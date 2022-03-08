#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate dotenv;

use crate::handler::http::middleware::cors::CORS;

pub mod application;
pub mod core;
pub mod infrastructure;
pub mod schema;
pub mod handler;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(CORS)
        .mount("/users/", handler::http::user_routes())
}
