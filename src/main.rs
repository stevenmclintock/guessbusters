use std::env;
use std::error::Error;
use dotenv::dotenv;
use reqwest::header::{ACCEPT, AUTHORIZATION, HeaderMap, HeaderValue};
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file
    dotenv().ok();
    
    let tmdb_api_key = env::var("TMDB_API_KEY")
        .expect("TMDB_API_KEY environment value not found");
    
    println!("tmdb_api_key is {}", tmdb_api_key);

    let auth_token = format!("Bearer {}", tmdb_api_key);
    
    let mut headers = HeaderMap::new();
    headers
        .insert(AUTHORIZATION, HeaderValue::from_str(&auth_token)
        .expect("Invalid authorization header value"));
    headers
        .insert(ACCEPT, HeaderValue::from_str("application/json")
        .expect("Invalid accept header value"));

    let client = reqwest::Client::new();

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

    let resp = client
        .get("https://api.themoviedb.org/3/discover/movie?include_adult=false&include_video=false&language=en-US&page=1&sort_by=popularity.desc")
        .headers(headers)
        .send()
        .await?
        .json::<Discover>()
        .await?;

    println!("total pages is {}", resp.total_pages);

    Ok(())
}
