use reqwest::Client;
use std::error::Error;
use rand::Rng;
pub mod headers;
pub mod discover;
pub mod credits;
pub mod genres;

pub struct RandomMovieDetails {
    pub metadata: discover::Metadata,
    pub credits: credits::Credits,
    pub multi_choice: [String; 2]
}

pub async fn random_movie_details (client: &Client, tmdb_api_key: &str) -> Result<RandomMovieDetails, Box<dyn Error>> {
    /*
     * Perform an initial "discover" request
     * to determine the total page count.
     */
    let initial_discover_resp = discover::Discover::get(&client, &tmdb_api_key, 1).await?;

    let multi_choice_1 = initial_discover_resp.results[0].title.clone();
    let multi_choice_2 = initial_discover_resp.results[1].title.clone();


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
    let metadata = 
        discover::Discover::get(&client, &tmdb_api_key, random_page)
            .await?
            .results[0]
            .clone();

    /*
     * Execute a "credits" request using the ID of the random
     * movie we've retrieved to fetch the cast and crew.
     */
    let credits = 
        credits::Credits::get(&client, &tmdb_api_key, metadata.id).await?;

    Ok(RandomMovieDetails { metadata, credits, multi_choice: [multi_choice_1, multi_choice_2] })
}