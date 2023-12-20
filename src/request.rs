/// Module contains get_headers() function.
use reqwest::header::HeaderMap;

/// Returns reqwest headermap.  Currently only returns the content_type header as application/json.
pub fn get_headers() -> HeaderMap {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::CONTENT_TYPE,
        "application/json".parse().unwrap(),
    );
    headers
}
