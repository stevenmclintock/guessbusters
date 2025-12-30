use std::env;
use std::error::Error;
use dotenv::dotenv;
use reqwest::Client;
mod tmdb;
mod questions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file
    dotenv().ok();

    let tmdb_api_key = env::var("TMDB_API_KEY")
        .expect("TMDB_API_KEY environment value not found");

    let client = Client::new();

    let genres = tmdb::genres::Genres::get(&client, &tmdb_api_key).await?.genres;

    let random_movie_details = tmdb::random_movie_details(&client, &tmdb_api_key).await?;

    println!("random movie metadata is {:?}", random_movie_details.metadata);
    println!("random movie credits is {:?}", random_movie_details.credits);

    println!("questions are: {:?}", questions::get_questions(&random_movie_details, &genres));

    Ok(())
}
