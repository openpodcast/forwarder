use crate::client::client;
use crate::helpers::upstream;
use crate::{forward::extract_ref, posthog};
use serde_json::json;
use worker::{Request, Result, RouteContext};

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
pub(crate) fn posthog<D>(request: &Request, ctx: &RouteContext<D>) -> Result<posthog::Event> {
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

/// Create `OpenPodcast API` event from Cloudflare request
pub(crate) fn openpodcast<D>(
    request: &Request,
    ctx: &RouteContext<D>,
) -> Result<serde_json::Value> {
    let (latitude, longitude) = request.cf().coordinates().unwrap_or((0.0, 0.0));
    // concatenate headers into a single string separated by semi-colons
    let headers = request
        .headers()
        .into_iter()
        .map(|(key, value)| format!("{}: {}", key, value))
        .collect::<Vec<String>>()
        .join("; ");

    let event = json!({
        "kind":request_kind(request.path()),
        "upstream": upstream(ctx)?,
        "upstream_ref": extract_ref(request).and_then(|s| Ok(s.to_string()))?,
        "client": client(request).name(),
        "is_bot": client(request).is_bot(),
        "cloudflare": format!("{:#?}", request.cf()),
        "country": request.cf().country(),
        "path": request.path(),
        "latitude": latitude,
        "longitude": longitude,
        "headers": headers,
        "user-agent": request.headers().get("user-agent").unwrap_or(None),
        "ip": request.headers().get("x-real-ip").unwrap_or(None),
    });

    Ok(event)
}
