use log::info;
use reqwest::header::HeaderMap;

/// Returns reqwest headermap. If the GCP_API_KEY environment variable is present, then it is added to the headers.
pub fn get_headers() -> HeaderMap {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::CONTENT_TYPE,
        "application/json".parse().unwrap(),
    );

    if let Ok(key) = dotenvy::var("GCP_API_KEY") {
        info!("Using the GCP API key...");
        headers.insert("X-goog-api-key", key.parse().unwrap());
    }

    headers
}
