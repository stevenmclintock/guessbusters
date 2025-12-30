use serde_derive::Deserialize;
use serde_derive::Serialize;
use reqwest::Client;
use std::error::Error;
use crate::tmdb::headers;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Discover {
    pub results: Vec<Metadata>,
    #[serde(rename = "total_pages")]
    pub total_pages: i64,
}

impl Discover {
    pub async fn get (client: &Client, tmdb_api_key: &str, mut page: i64) -> Result<Discover, Box<dyn Error>> {
        let params = [
            ("include_adult", "false"),
            ("include_video", "false"),
            ("language", "en-US"),
            ("sort_by", "popularity.desc"),
            ("vote_average.gte", "5"),
            ("with_original_language", "en"),
            ("without_genres", "99,36,10402,10770")
        ];

        /*
        * The TMDB API does not let you pass in a "page" 
        * parameter that is greater than 500.
        */
        page = if page > 500 { 500 } else { page };

        Ok(
            client
                .get(format!("https://api.themoviedb.org/3/discover/movie?page={}", page.to_string()))
                .query(&params)
                .headers(headers::default_headers(&tmdb_api_key))
                .send()
                .await?
                .json::<Discover>()
                .await?
        )
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    #[serde(rename = "genre_ids")]
    pub genre_ids: Vec<i64>,
    pub id: i64,
    pub overview: String,
    #[serde(rename = "poster_path")]
    pub poster_path: Option<String>,
    #[serde(rename = "release_date")]
    pub release_date: String,
    pub title: String,
}