//! Edge worker for handling and modifying RSS feeds on the fly
//! based on user agents
mod forward;
mod panic;
mod rss;
mod user_agent;
use worker::*;

const UPSTREAM_FEED_URL: &str = "https://feeds.megaphone.fm/GLT2733274547";

use crate::rss::Replacer;

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or_else(|| "unknown region".into())
    );
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    log_request(&req);

    // Get more helpful error messages written to the console in the case of a panic.
    panic::set_panic_hook();

    // Use the Router to handle matching endpoints, use ":name" placeholders, or "*name"
    // catch-alls to match on specific patterns. Alternatively, use `Router::with_data(D)` to
    // provide arbitrary data that will be accessible in each route via the `ctx.data()` method.
    let router = Router::new();

    // Each route will get a `Request` for handling HTTP
    // functionality and a `RouteContext` which you can use to get route parameters and
    // Environment bindings like KV Stores, Durable Objects, Secrets, and Variables.
    router
        .get_async("/", |request, _| async {
            let user_agent = user_agent::from(request);
            let user_agent = match user_agent {
                Ok(ua) => ua,
                Err(e) => {
                    // Silently ignore user agent errors to avoid breaking requests
                    println!("Error detecting user agent: {e}");
                    "unknown".to_string()
                }
            };
            // TODO: Add proper logging for user agents
            println!("Received request form {user_agent}");

            // // Fetch original RSS feed
            let mut orig_response = Fetch::Request(Request::new(UPSTREAM_FEED_URL, Method::Get)?)
                .send()
                .await?;
            let feed_content = orig_response.text().await?;

            // Rewrite original feed with edge worker URLs, but keep original mp3 URLs
            // and attach them as encoded string for future forwarding
            let output = Replacer::new("worker.mre.workers.dev", Some("/r")).replace(feed_content);
            // let response = Response::ok(output)?.with_headers(orig_response.headers().to_owned());
            let response = Response::ok(output)?;
            Ok(response)
        })
        .get("/r/*forward_url", |request, _context| {
            // let cookie = request.headers().get("Cookie")?;
            match forward::get(request, Some("/r")) {
                Ok(url) => {
                    println!("Forwarding to {url}");
                    let response = Response::redirect(url)?;
                    // if cookie.is_none() {
                    // let headers = response.headers_mut();
                    // doesn't work yet...
                    // headers.set("Set-Cookie", "id=a3fWa; SameSite=None")?;
                    // }
                    Ok(response)
                }
                Err(e) => Response::error(e.to_string(), 404),
            }
        })
        .get("/version", |_, ctx| {
            let version = ctx.var("WORKERS_RS_VERSION")?.to_string();
            Response::ok(version)
        })
        .run(req, env)
        .await
}
