//! Edge worker for handling and modifying RSS feeds on the fly
//! based on user agents

#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![allow(clippy::future_not_send)]

mod forward;
mod helpers;
mod panic;
mod posthog;
mod rss;
mod user_agent;

use crate::rss::Replacer;
use helpers::{host, log_request, upstream};
use user_agent::user_agent;
use worker::{console_log, event, Env, Fetch, Method, Request, Response, Result, Router};

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
        .get_async("/", |request, ctx| async move {
            let upstream = &upstream(&ctx)?;
            let user_agent = user_agent(&request);
            console_log!("Received request from {user_agent}");

            let response = posthog::Client::new(ctx.var("POSTHOG_API_KEY")?.to_string())
                .send(posthog::Event::new("rss", upstream).property("user_agent", user_agent)?)
                .await?;
            console_log!("PostHog status: {:#?}", response);

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
                    let mut event = posthog::Event::new("mp3", &upstream(&ctx)?)
                        .property("user_agent", user_agent(&request))?
                        .property("path", request.path())?;

                    if let Some(reference) = url.query_pairs().find(|(k, _)| k == "ref") {
                        event = event.property("upstream", reference)?;
                    }

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
