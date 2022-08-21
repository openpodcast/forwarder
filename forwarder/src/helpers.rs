use worker::{console_log, Date, Request, Result, RouteContext};

/// Log request information
pub fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, cf: {:#?}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf()
    );
}

/// Get the worker host URL (e.g. <worker.namespace.workers.dev>)
pub fn host(req: &Request) -> Result<String> {
    let url = req.url()?;
    let host = url
        .host()
        .ok_or_else(|| worker::Error::RustError("Cannot get worker host".to_string()))?;
    Ok(host.to_string())
}

/// Get the feed URL from the worker config
pub fn upstream<D>(ctx: &RouteContext<D>) -> Result<String> {
    Ok(ctx.var("UPSTREAM_FEED_URL")?.to_string())
}
