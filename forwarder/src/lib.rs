//! Edge worker for handling and modifying RSS feeds on the fly
//! based on user agents

#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![allow(clippy::future_not_send)]

mod client;
mod forward;
mod helpers;
mod panic;
mod posthog;
mod rss;

use crate::{forward::extract_ref, rss::Replacer};
use client::client;
use helpers::{host, log_request, upstream};
use worker::{
    console_log, event, Env, Fetch, Method, Request, Response, Result, RouteContext, Router,
};

fn request_kind(path: String) -> String {
    if path == "/" {
        "rss".to_string()
    } else if path.starts_with("/r/") {
        "mp3".to_string()
    } else {
        path
    }
}

/// Create `PostHog` event from Cloudflare request
fn create_cloudflare_event<D>(request: &Request, ctx: &RouteContext<D>) -> Result<posthog::Event> {
    let kind = request_kind(request.path());
    let mut event = posthog::Event::new(&kind, &upstream(ctx)?)
        .property("client", client(request).name())?
        .property("is_bot", client(request).is_bot())?
        .property("cloudflare", format!("{:#?}", request.cf()))?
        .property("country", request.cf().country())?
        .property("path", request.path())?;

    if let Some((latitude, longitude)) = request.cf().coordinates() {
        event = event
            .property("latitude", latitude)?
            .property("longitude", longitude)?;
    }
    for (key, value) in request.headers() {
        event = event.property(key, value)?;
    }
    // Overwrite ip for GeoIP lookup
    if let Ok(ip) = request.headers().get("x-real-ip") {
        event = event.property("$ip", ip)?;
    }

    // Upstream ref is only set for mp3 requests
    if let Ok(reference) = extract_ref(request) {
        event = event.property("upstream", reference.as_ref())?;
    }
    Ok(event)
}

/// Handle RSS feed requests by forwarding them to the original URL and logging
/// the request
///
/// # Errors
///
/// * the request is not a valid RSS feed request
/// * the request could not be forwarded
/// * the feed URL could not be retrieved from the config
#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    log_request(&req);

    // Get more helpful error messages written to the console in the case of a
    // panic.
    panic::set_panic_hook();

    // Use the Router to handle matching endpoints, use ":name" placeholders, or
    // "*name" catch-alls to match on specific patterns. Alternatively, use
    // `Router::with_data(D)` to provide arbitrary data that will be accessible
    // in each route via the `ctx.data()` method.
    let router = Router::new();

    // Each route will get a `Request` for handling HTTP functionality and a
    // `RouteContext` which you can use to get route parameters and Environment
    // bindings like KV Stores, Durable Objects, Secrets, and Variables.
    router
        // Request for RSS feed
        .get_async("/", |request, ctx| async move {
            let upstream = &upstream(&ctx)?;
            let client = client(&request);
            console_log!("Received request from {}", client.name());

            // Comment out the following line to enable RSS feed logging in PostHog
            // let event = create_cloudflare_event(&request, &ctx)?;
            // let response = posthog::Client::new(ctx.var("POSTHOG_API_KEY")?.to_string())
            //     .send(event)
            //     .await?;
            // console_log!("PostHog status: {:#?}", response);

            // Fetch original RSS feed.
            let mut orig_response = Fetch::Request(Request::new(upstream, Method::Get)?)
                .send()
                .await?;
            let feed_content = orig_response.text().await?;

            // Rewrite original feed with edge worker URLs, but keep original
            // mp3 URLs and attach them as encoded string for future forwarding
            let output = Replacer::new(host(&request)?, Some("/r")).replace(feed_content);

            // Pass original request headers to client
            let mut response = Response::ok(output)?.with_headers(orig_response.headers().clone());

            response
                .headers_mut()
                .append("Set-Cookie", "forwarder=bar; SameSite=None")?;

            Ok(response)
        })
        .get_async("/r/*forward_url", |request, ctx| async move {
            match forward::get(&request, Some("/r")) {
                Ok(url) => {
                    let event = create_cloudflare_event(&request, &ctx)?;
                    let response = posthog::Client::new(ctx.var("POSTHOG_API_KEY")?.to_string())
                        .send(event)
                        .await?;
                    console_log!("PostHog status: {:#?}", response);

                    println!("Forwarding to {url}");
                    let response = Response::redirect(url)?;

                    // Clone the response so that it's no longer immutable
                    // https://community.cloudflare.com/t/how-to-modify-immutable-headers-and-add-nonces-to-response-header/146165
                    // https://developers.cloudflare.com/workers/examples/alter-headers
                    let mut new_response = Response::from_body(response.body().clone())?;

                    new_response
                        .headers_mut()
                        .append("Set-Cookie", "forwarder=bar; SameSite=None")?;
                    Ok(response)
                }
                Err(e) => Response::error(e.to_string(), 404),
            }
        })
        .get("/version", |_, ctx| {
            let version = ctx.var("VERSION")?.to_string();
            Response::ok(version)
        })
        .run(req, env)
        .await
}
