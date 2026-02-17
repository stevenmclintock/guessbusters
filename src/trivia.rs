use std::env;
use rand::seq::SliceRandom;
use rand::{rng};
use reqwest::Client;
use std::error::Error;

pub mod tmdb;

const NOT_ENOUGH_INFORMATION: &str = "Oops! We didn't have enough information for this question.";

pub struct Trivia {
    pub questions: [String; 4],
    pub answer: String
}

impl Trivia {
    pub async fn get() -> Result<Trivia, Box<dyn Error>> {
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

        Ok(
            Trivia {
                questions: [
                    Self::get_question_1(&random_movie, &genres),
                    Self::get_question_2(&random_movie.credits.cast),
                    Self::get_question_3(&random_movie.metadata.overview),
                    Self::get_question_4(&random_movie.multi_choice)
                ],
                answer: random_movie.metadata.title
            }
        )
    }

    pub fn check(guess: &String, answer: &String) -> bool {
        guess.trim().to_lowercase() == answer.trim().to_lowercase()
    }

    fn get_question_1 (random_movie: &tmdb::RandomMovie, genres: &Vec<tmdb::genres::Genre>) -> String {
        let genres = genres
            .into_iter()
            .filter(|genre| random_movie.metadata.genre_ids.contains(&genre.id))
            .map(|crew| crew.name.clone())
            .collect::<Vec<String>>()
            .join(", ");
        
        let directors = random_movie.credits.crew
            .iter()
            .filter(|crew| crew.job.to_lowercase() == "director")
            .map(|crew| crew.name.clone())
            .collect::<Vec<String>>()
            .join(", ");

        let release_date = &random_movie.metadata.release_date;

        return 
            if genres.len() > 0 && release_date.len() > 0 && directors.len() > 0 {
                format!("This movie is a {} and was released on {}. It was directed by {}.", genres, release_date, directors)
            } else if genres.len() > 0 && release_date.len() > 0 {
                format!("This movie is a {} and was released on {}.", genres, release_date)
            } else if release_date.len() > 0 && directors.len() > 0 {
                format!("This movie was released on {} and was directed by {}.", release_date, directors)
            } else if genres.len() > 0 && directors.len() > 0 {
                format!("This movie is a {} and was directed by {}.", genres, directors)
            } else if genres.len() > 0 {
                format!("This movie is a {}.", genres)
            } else if release_date.len() > 0 {
                format!("This movie was released on {}.", release_date)
            } else if directors.len() > 0 {
                format!("This movie was directed by {}.", directors)
            } else {
                NOT_ENOUGH_INFORMATION.to_string()
            };
    }

    fn get_question_2 (cast: &Vec<tmdb::credits::Cast>) -> String {
        let cast = cast
            .into_iter()
            .filter(|cast| cast.known_for_department.to_lowercase() == "acting" && !cast.name.is_empty())
            .take(3)
            .map(|cast| {
                if !cast.character.is_empty() {
                    format!("{} as {}", cast.name, cast.character)
                } else {
                    cast.name.clone()
                }
            }).collect::<Vec<String>>()
            .join(", ");

        return
            if cast.len() > 0 {
                format!("This movie features {}.", cast)
            } else {
                NOT_ENOUGH_INFORMATION.to_string()
            };
    }

    fn get_question_3 (overview: &String) -> String {
        if overview.len() == 0 {
            return NOT_ENOUGH_INFORMATION.to_string()
        }

        return format!("The synopsis of this movie is: {}", overview);
    }

    fn get_question_4 (multi_choice: &[String; 3]) -> String {
        if multi_choice.len() != 3 {
            return NOT_ENOUGH_INFORMATION.to_string()
        }

        let mut random_multi_choice = multi_choice.clone();
        let mut rng = rng();
        random_multi_choice.shuffle(&mut rng);

        return format!(
            "This movie is either {}, {} or {}.",
            random_multi_choice[0],
            random_multi_choice[1],
            random_multi_choice[2]);
    }
}