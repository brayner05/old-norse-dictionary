use rocket::{get, Route, routes};
use rocket_dyn_templates::{context, Template};


#[get("/")]
fn admin() -> Template {
    Template::render("admin", context! { })
}


pub fn routes() -> Vec<Route> {
    routes![admin]
}