use crate::tmdb::RandomMovieDetails;
use crate::tmdb::genres::Genre;

pub fn get_questions (random_movie_details: &RandomMovieDetails, genres: &Vec<Genre>) -> Vec<String> {
    let mut questions: Vec<String> = Vec::new();

    let genres = genres
                                .into_iter()
                                .filter(|genre| random_movie_details.metadata.genre_ids.contains(&genre.id))
                                .map(|crew| crew.name.clone())
                                .collect::<Vec<String>>()
                                .join(", ");
    
    let directors = random_movie_details.credits.crew
                                .iter()
                                .filter(|crew| crew.job.to_lowercase() == "director")
                                .map(|crew| crew.name.clone())
                                .collect::<Vec<String>>()
                                .join(", ");

    let release_date = &random_movie_details.metadata.release_date;

    let question_1 = 
        if genres.len() > 0 && release_date.len() > 0 && directors.len() > 0 {
            format!("This movie is a {} and was released in {}. It was directed by {}.", genres, release_date, directors)
        } else if genres.len() > 0 && release_date.len() > 0 {
            format!("This movie is a {} and was released in {}.", genres, release_date)
        } else if release_date.len() > 0 && directors.len() > 0 {
             format!("This movie was released in {} and was directed by {}.", release_date, directors)
        } else if genres.len() > 0 && directors.len() > 0 {
             format!("This movie is a {} and was directed by {}.", genres, directors)
        } else if genres.len() > 0 {
             format!("This movie is a {}.", genres)
        } else if release_date.len() > 0 {
            format!("This movie was released in {}.", release_date)
        } else if directors.len() > 0 {
            format!("This movie was directed by {}.", directors)
        } else {
            "Oops! We didn't have enough information for this question.".to_string()
        };

    questions.push(question_1);

    return questions;
}