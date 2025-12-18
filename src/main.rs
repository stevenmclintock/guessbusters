use std::env;
use std::error::Error;
use dotenv::dotenv;
use reqwest::Client;
mod tmdb;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file
    dotenv().ok();

    let tmdb_api_key = env::var("TMDB_API_KEY")
        .expect("TMDB_API_KEY environment value not found");

    let client = Client::new();

    let resp = tmdb::random_movie_details(&client, &tmdb_api_key).await?;

    println!("random movie is {:?}", resp);

    Ok(())
}
