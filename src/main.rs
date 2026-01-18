use std::env;
use std::error::Error;
use std::io::stdin;
use dotenv::dotenv;
use reqwest::Client;
mod tmdb;
mod questions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file
    dotenv().ok();

    // Retrieve the TMDB API key from .env file
    let tmdb_api_key = env::var("TMDB_API_KEY")
        .expect("TMDB_API_KEY environment value not found");

    /*
     * Retrieve a list of all movie genres using the TMDB API,
     * in addition to a random movie for the user to guess.
     */
    let client = Client::new();
    let genres = tmdb::genres::Genres::get(&client, &tmdb_api_key).await?.genres;
    let random_movie = tmdb::RandomMovie::get(&client, &tmdb_api_key).await?;

    println!("Welcome to Guessbusters!");
    println!("4 questions. 4 chances. Can you guess the random movie?");

    // Retrieve the questions
    let questions = questions::get_questions(&random_movie, &genres);

    let mut winner = false;
    
    for question in questions.iter() {
        println!("{}", question);
        println!("Please enter your guess. Or enter P to pass:");

        // Prompt the user for their guess
        let mut console_input = String::new();
        stdin().read_line(&mut console_input).expect("Oops! Something went wrong.");

        // Normalize the console input and random movie
        let normalized_console_input = console_input.trim().to_lowercase();
        let normalized_random_movie = random_movie.metadata.title.trim().to_lowercase();

        // Determine if the guess is correct
        winner = match normalized_console_input.as_str() {
            "p" => false,
            guess if guess == normalized_random_movie => true,
            _ => false
        };

        /*
         * Don't ask any further questions if the guess was correct,
         * otherwise inform the user they were incorrect.
         */
        if winner {
            break;
        } else if normalized_console_input == "p" {
            println!("Ok. I'll skip that one.");
        } else {
            println!("Nope! That was incorrect.");
        }
    }

    if winner {
        println!("Congratulations! You truly are a Guessbuster!");
    } else {
        println!("Sorry! You are not a Guessbuster this time.");
        println!("The movie you were trying to guess was: {}", random_movie.metadata.title);
    }

    Ok(())
}
