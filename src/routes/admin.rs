use diesel::{query_dsl::methods::SelectDsl, RunQueryDsl, SelectableHelper};
use rocket::{get, Route, routes, State};
use rocket_dyn_templates::{context, Template};

use crate::{database::DbPool, models::word::Word};


#[get("/")]
fn admin(pool: &State<DbPool>) -> Template {
    use crate::schema::word::dsl::word;
    let mut db_connection = pool.get()
            .expect("Failed to retrieve database connection from database pool.");
        
    let words = word.select(Word::as_select())
            .load::<Word>(&mut db_connection)
            .expect("Failed to load posts.");

    Template::render("admin", context! { })
}


pub fn routes() -> Vec<Route> {
    routes![admin]
}