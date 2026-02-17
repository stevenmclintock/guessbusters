use std::error::Error;
use std::io::stdin;
use dotenv::dotenv;

mod trivia;

const SKIP_QUESTION_KEY: &str = "S";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file
    dotenv().ok();

    // Retrieve the trivia (e.g. the questions and the answer)
    let trivia = trivia::Trivia::get().await?;

    println!("Welcome to Guessbusters!");
    println!("4 questions. 4 chances. Can you guess the random movie?");

    let mut winner = false;
    
    for question in trivia.questions.iter() {
        println!("{}", question);
        println!("Please enter your guess. Or enter {} to skip to the next question:", SKIP_QUESTION_KEY);

        // Prompt the user for their guess
        let mut console_input = String::new();
        stdin().read_line(&mut console_input).expect("Oops! Something went wrong.");

        // Determine if the guess is correct
        winner = match console_input.trim() {
            SKIP_QUESTION_KEY => false,
            guess if trivia::Trivia::check(&guess.to_string(), &trivia.answer) => true,
            _ => false
        };

        /*
         * Don't ask any further questions if the guess was correct,
         * otherwise inform the user they were incorrect.
         */
        if winner {
            break;
        } else if console_input.trim() == SKIP_QUESTION_KEY {
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
