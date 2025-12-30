use serde_derive::Deserialize;
use serde_derive::Serialize;
use reqwest::Client;
use std::error::Error;
use crate::tmdb::headers;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Genres {
    pub genres: Vec<Genre>,
}

impl Genres {
    pub async fn get (client: &Client, tmdb_api_key: &str) -> Result<Genres, Box<dyn Error>> {
        Ok(
            client
                .get("https://api.themoviedb.org/3/genre/movie/list")
                .headers(headers::default_headers(&tmdb_api_key))
                .send()
                .await?
                .json::<Genres>()
                .await?
        )
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Genre {
    pub id: i64,
    pub name: String,
}