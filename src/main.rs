use rocket::{get, routes};


#[get("/")]
fn index() -> &'static str {
    "Hello, World!"
}


#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    rocket::build()
        .mount("/", routes![index])
        .launch()
        .await?;

    Ok(())
}
