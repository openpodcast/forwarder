//! Edge worker for handling and modifying RSS feeds on the fly
//! based on user agents

#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![allow(clippy::future_not_send)]

mod forward;
mod panic;
mod posthog;
mod rss;
mod user_agent;
use worker::{console_log, event, Date, Env, Fetch, Method, Request, Response, Result, Router};

use crate::rss::Replacer;

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, cf: {:#?}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf()
    );
}

/// Handle RSS feed requests by forwarding them to the original URL and logging
/// the request
///
/// # Errors
///
/// * the request is not a valid RSS feed request
/// * the request could not be forwarded
/// * the feed URL could not be retr&ieved from the config
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
            // Get feed URL from worker environment.
            let upstream = ctx.var("UPSTREAM_FEED_URL")?.to_string();

            let user_agent = user_agent::from(&request);
            let user_agent = match user_agent {
                Ok(ua) => ua,
                Err(e) => {
                    // Silently ignore user agent errors to avoid breaking
                    // requests
                    console_log!("Error detecting user agent: {e}");
                    "unknown".to_string()
                }
            };
            console_log!("Received request from {user_agent}");

            let client = posthog::Client::new(ctx.var("POSTHOG_API_KEY")?.to_string());
            let response = client
                .send(posthog::Event::new("rss", &upstream).property("user_agent", user_agent)?)
                .await?;
            console_log!("PostHog status: {:#?}", response);

            // Fetch original RSS feed.
            let mut orig_response = Fetch::Request(Request::new(&upstream, Method::Get)?)
                .send()
                .await?;
            let feed_content = orig_response.text().await?;

            // Rewrite original feed with edge worker URLs, but keep original
            // mp3 URLs and attach them as encoded string for future forwarding
            let output =
                Replacer::new("forwarder.mre.workers.dev", Some("/r")).replace(feed_content);

            // Pass original request headers to client
            let mut response = Response::ok(output)?.with_headers(orig_response.headers().clone());

            console_log!("Set cookie");
            response
                .headers_mut()
                .append("Set-Cookie", "forwarder=bar; SameSite=None")?;

            Ok(response)
        })
        .get("/r/*forward_url", |request, _ctx| {
            let cookie = request.headers().get("Cookie")?;
            console_log!("{cookie:?}");
            match forward::get(&request, Some("/r")) {
                Ok(url) => {
                    println!("Forwarding to {url}");
                    let response = Response::redirect(url)?;

                    // Clone the response so that it's no longer immutable
                    // https://community.cloudflare.com/t/how-to-modify-immutable-headers-and-add-nonces-to-response-header/146165
                    // https://developers.cloudflare.com/workers/examples/alter-headers
                    let mut new_response = Response::from_body(response.body().clone())?;

                    console_log!("Set cookie");
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
