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

/// Get the feed URL from the worker config
pub fn upstream<D>(ctx: &RouteContext<D>) -> Result<String> {
    Ok(ctx.var("UPSTREAM_FEED_URL")?.to_string())
}
