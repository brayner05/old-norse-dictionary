use std::env;

use database::{establish_pool, DbPool};
use diesel::{Connection, SqliteConnection};
use dotenvy::dotenv;
use rocket_dyn_templates::Template;

pub mod database;
pub mod models;
pub mod routes;
pub mod schema;


fn initialize_database() -> DbPool {
    // Load .env file
    dotenv().ok();
    
    // Read DATABASE_URL from .env
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set!");

    // Attempt to connect to the database
    establish_pool(&database_url)
}


#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // Connect to the database
    let db_pool = initialize_database();

    // Initialize Rocket
    rocket::build()
        .manage(db_pool)
        .mount("/admin", routes::admin::routes())
        .attach(Template::fairing())
        .launch()
        .await?;

    Ok(())
}
