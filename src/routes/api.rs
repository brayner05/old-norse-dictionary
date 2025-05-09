use rocket::{get, routes, Route, State};
use rocket::serde::json::Json;

use crate::repositories::word_repository;
use crate::dtos::WordDto;
use crate::{database::DbPool, models::word::Word};


#[get("/words")]
fn get_all_words(pool: &State<DbPool>) -> Json<Vec<Word>> {
    let words = word_repository::get_all_words(pool, None);
    Json(words)
}


#[get("/words/<word>")]
fn get_word(pool: &State<DbPool>, word: String) -> Json<Option<WordDto>> {
    let result = word_repository::get_word_definitions(pool, &word);
    Json(result)
}


pub fn routes() -> Vec<Route> {
    routes![get_all_words, get_word]
}