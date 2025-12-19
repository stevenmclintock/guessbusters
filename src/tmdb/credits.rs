use serde_derive::Deserialize;
use serde_derive::Serialize;
use reqwest::Client;
use std::error::Error;
use crate::tmdb::headers;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Credits {
    pub cast: Vec<Cast>,
    pub crew: Vec<Crew>,
}

impl Credits {
    pub async fn get (client: &Client, tmdb_api_key: &str, movie_id: i64) -> Result<Credits, Box<dyn Error>> {
        Ok(
            client
                .get(format!("https://api.themoviedb.org/3/movie/{}/credits", movie_id.to_string()))
                .headers(headers::default_headers(&tmdb_api_key))
                .send()
                .await?
                .json::<Credits>()
                .await?
        )
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cast {
    #[serde(rename = "known_for_department")]
    pub known_for_department: String,
    pub name: String,
    pub character: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Crew {
    pub name: String,
    pub job: String,
}