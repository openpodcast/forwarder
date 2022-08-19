use urlencoding::decode;
use worker::*;

/// Check if the given request URL points to a valid mp3 file
fn valid_forwarding_url(request: &Request, prefix: &str) -> Result<()> {
    // Sanity checks to see if this is a valid forwarding URL
    let url = request.url()?;
    let path = url.path();
    if !path.starts_with(prefix) {
        return Err(Error::RustError(format!(
            "Forward URL does not start with `{prefix}` prefix"
        )));
    }
    if !path.ends_with("mp3") && !path.ends_with("aac") {
        return Err(Error::RustError(format!(
            "Unknown audio file format: {path}"
        )));
    };
    Ok(())
}

/// Extract redirect URL from ref parameter
fn extract_ref(request: &Request) -> Result<Url> {
    let html_decoded = html_escape::decode_html_entities(&request.url()?).to_string();
    let url = Url::parse(&html_decoded)?;
    if let Some((_, forward)) = url.query_pairs().find(|(k, _)| k == "ref") {
        let decoded = decode(&forward)
            .map_err(|e| Error::RustError(format!("Cannot decode ref {forward}: {e}")))?;
        return Ok(Url::parse(&decoded)?);
    };
    Err(Error::RustError("Could not find ref parameter".to_string()))
}

/// Extract our custom forward URL form the request.
/// It is encoded in the `ref` query parameter
/// Example:
/// https://example.org/r/podcast1.mp3?ref=https%253A%252F%252Fexample.com%252Fpodcast1.mp3
pub fn get(request: Request, prefix: Option<&str>) -> Result<Url> {
    if let Some(prefix) = prefix {
        valid_forwarding_url(&request, prefix)?;
    }
    extract_ref(&request)
}
