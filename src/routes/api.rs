use diesel::{query_dsl::methods::SelectDsl, RunQueryDsl, SelectableHelper};
use rocket::{get, routes, Route, State};
use rocket::serde::json::Json;

use crate::{database::DbPool, models::word::Word};


#[get("/word")]
fn get_all_words(pool: &State<DbPool>) -> Json<Vec<Word>> {
    use crate::schema::word::dsl::word;

    let mut db_connection = pool.get()
        .expect("Could not find a database connection");

    let words = word.select(Word::as_select())
        .load::<Word>(&mut db_connection)
        .unwrap_or(vec![]);

    Json(words)
}


pub fn routes() -> Vec<Route> {
    routes![get_all_words]
}