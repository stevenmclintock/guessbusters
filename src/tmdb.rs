use reqwest::Client;
use std::error::Error;
use rand::Rng;
pub mod headers;
pub mod discover;

pub async fn random_movie_details (client: &Client, tmdb_api_key: &str) -> Result<discover::Movie, Box<dyn Error>> {
    /*
     * Perform an initial "discover" request
     * to determine the total page count.
     */
    let initial_discover_resp = discover::Discover::get(&client, &tmdb_api_key, 1).await?;

    /*
     * Generate a random number between 1 and 
     * the total page count to provide a truly
     * random movie for the user to guess!
     */
    let mut rng = rand::rng();
    let random_page = rng.random_range(1..initial_discover_resp.total_pages);

    /*
     * Execute another "discover" request using
     * the random page and return the first result
     * to use as the random movie.
     */
    Ok(
        discover::Discover::get(&client, &tmdb_api_key, random_page).await?.results[0].clone()
    )
}