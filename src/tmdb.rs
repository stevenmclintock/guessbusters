use reqwest::Client;
use std::{error::Error};
use rand::Rng;
pub mod headers;
pub mod discover;
pub mod credits;
pub mod genres;

pub struct RandomMovie {
    pub metadata: discover::Metadata,
    pub credits: credits::Credits,
    pub multi_choice: [String; 3]
}

impl RandomMovie {
    pub async fn get (client: &Client, tmdb_api_key: &str) -> Result<RandomMovie, Box<dyn Error>> {
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



        // Execute another "discover" request using the random page
        let random_discover_resp = discover::Discover::get(&client, &tmdb_api_key, random_page).await?;

        // Return the first result to use as the random movie
        let metadata = random_discover_resp.results[0].clone();
        
        // Retrieve the 3 movie titles to use as the multiple choice question
        let multi_choice = if random_discover_resp.results.len() >= 3 {
                [
                    metadata.title.clone(),
                    random_discover_resp.results[1].title.clone(),
                    random_discover_resp.results[2].title.clone(),
                    
                ]
            } else {
                [
                    metadata.title.clone(),
                    initial_discover_resp.results[0].title.clone(),
                    initial_discover_resp.results[1].title.clone(),
                ]
            };

        /*
        * Execute a "credits" request using the ID of the random
        * movie we've retrieved to fetch the cast and crew.
        */
        let credits = 
            credits::Credits::get(&client, &tmdb_api_key, metadata.id).await?;

        Ok(RandomMovie { metadata, credits, multi_choice })
    }
}