use rand::seq::SliceRandom;
use rand::{rng};

use crate::tmdb::RandomMovie;
use crate::tmdb::credits::Cast;
use crate::tmdb::genres::Genre;

const NOT_ENOUGH_INFORMATION: &str = "Oops! We didn't have enough information for this question.";

pub fn get_questions (random_movie: &RandomMovie, genres: &Vec<Genre>) -> [String; 4] {
    return [
        get_question_1(&random_movie, &genres),
        get_question_2(&random_movie.credits.cast),
        get_question_3(&random_movie.metadata.overview),
        get_question_4(&random_movie.multi_choice)
    ];
}

fn get_question_1 (random_movie: &RandomMovie, genres: &Vec<Genre>) -> String {
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

fn get_question_2 (cast: &Vec<Cast>) -> String {
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