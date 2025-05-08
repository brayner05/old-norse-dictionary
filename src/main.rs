use std::{env, process::Command};

use database::{establish_pool, DbPool};
use dotenvy::dotenv;
use rocket::fs::{FileServer, relative};
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


fn configure_application() {
    Command::new("sass")
        .arg("static/sass/main.sass")
        .arg("static/css/main.css")
        .output()
        .expect("Failed to compiled SASS");
}


#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // Configure the application before running.
    configure_application();

    // Connect to the database
    let db_pool = initialize_database();

    // Initialize Rocket
    rocket::build()
        .manage(db_pool)
        .mount("/admin", routes::admin::routes())
        .mount("/static", FileServer::from(relative!("/static")))
        .attach(Template::fairing())
        .launch()
        .await?;

    Ok(())
}
