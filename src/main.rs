use std::env;
use dotenv::dotenv;

fn main() {
    // Load environment variables from .env file
    dotenv().ok();
    
    let tmdb_api_key = env::var("TMDB_API_KEY").expect("$TMDB_API_KEY is not set");
    println!("tmdb_api_key is {}", tmdb_api_key);
}
