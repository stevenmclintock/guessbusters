use reqwest::header::{ACCEPT, AUTHORIZATION, HeaderMap, HeaderValue};

pub fn default_headers(tmdb_api_key: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();

    headers
        .insert(AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", tmdb_api_key))
        .expect("Invalid authorization header value"));
    headers
        .insert(ACCEPT, HeaderValue::from_str("application/json")
        .expect("Invalid accept header value"));

    return headers;
}