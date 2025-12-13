use std::env;
use std::error::Error;
use dotenv::dotenv;
mod tmdb;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file
    dotenv().ok();

    let tmdb_api_key = env::var("TMDB_API_KEY")
        .expect("TMDB_API_KEY environment value not found");

    let client = reqwest::Client::new();

    let resp = tmdb::discover::Discover::get(&client, &tmdb_api_key, 1).await?;

    println!("total pages is {}", resp.total_pages);

    Ok(())
}
