use reqwest::Client;
use reqwest::header::{ACCEPT, AUTHORIZATION, HeaderMap, HeaderValue};
use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::error::Error;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Discover {
    pub page: i64,
    pub results: Vec<DiscoverResult>,
    #[serde(rename = "total_pages")]
    pub total_pages: i64,
    #[serde(rename = "total_results")]
    pub total_results: i64,
}

impl Discover {
    pub async fn get (client: &Client, tmdb_api_key: &str, page: i64) -> Result<Discover, Box<dyn Error>> {
        let mut headers = HeaderMap::new();
        headers
            .insert(AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", tmdb_api_key))
            .expect("Invalid authorization header value"));
        headers
            .insert(ACCEPT, HeaderValue::from_str("application/json")
            .expect("Invalid accept header value"));

        let page_as_parameter = page.to_string();

        let params = [
            ("include_adult", "false"),
            ("include_video", "false"),
            ("language", "en-US"),
            ("page", &page_as_parameter),
            ("sort_by", "popularity.desc")
        ];

        Ok(
            client
                .get("https://api.themoviedb.org/3/discover/movie")
                .query(&params)
                .headers(headers)
                .send()
                .await?
                .json::<Discover>()
                .await?
        )
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiscoverResult {
    pub adult: bool,
    #[serde(rename = "backdrop_path")]
    pub backdrop_path: String,
    #[serde(rename = "genre_ids")]
    pub genre_ids: Vec<i64>,
    pub id: i64,
    #[serde(rename = "original_language")]
    pub original_language: String,
    #[serde(rename = "original_title")]
    pub original_title: String,
    pub overview: String,
    pub popularity: f64,
    #[serde(rename = "poster_path")]
    pub poster_path: String,
    #[serde(rename = "release_date")]
    pub release_date: String,
    pub title: String,
    pub video: bool,
    #[serde(rename = "vote_average")]
    pub vote_average: f64,
    #[serde(rename = "vote_count")]
    pub vote_count: i64,
}