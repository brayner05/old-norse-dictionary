use rocket::{get, routes, Route, State};
use rocket_dyn_templates::{context, Template};

use crate::{database::DbPool, repositories::word_repository};


#[get("/<word>")]
async fn search_word(pool: &State<DbPool>, word: &str) -> Option<Template> {
    let word = word_repository::get_word_definitions(pool, word);
    if word.is_none() {
        return None;
    }

    Some(Template::render("word", context! { word: word }))
}   


pub fn routes() -> Vec<Route> {
    routes![search_word]
}