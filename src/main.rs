use std::error::Error;
use std::io::stdin;
use dotenv::dotenv;

mod trivia;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file
    dotenv().ok();

    println!("Welcome to Guessbusters!");
    println!("4 questions. 4 chances. Can you guess the random movie?");

    // Retrieve the trivia (e.g. the questions and the answer)
    let trivia = trivia::Trivia::get().await?;

    let mut winner = false;
    
    for question in trivia.questions.iter() {
        println!("{}", question);
        println!("Please enter your guess. Or enter S to skip to the next question:");

        // Prompt the user for their guess
        let mut console_input = String::new();
        stdin().read_line(&mut console_input).expect("Oops! Something went wrong.");

        // Normalize the console input and random movie
        let normalized_console_input = console_input.trim().to_lowercase();
        let normalized_random_movie = trivia.answer.trim().to_lowercase();

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
        } else if normalized_console_input == "s" {
            println!("Ok. I'll skip that one.");
        } else {
            println!("Nope! That was incorrect.");
        }
    }

    if winner {
        println!("Congratulations! You truly are a Guessbuster!");
    } else {
        println!("Sorry! You are not a Guessbuster this time.");
        println!("The movie you were trying to guess was: {}", trivia.answer);
    }

    Ok(())
}
