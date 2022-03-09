use rocket_cors::{AllowedHeaders, AllowedOrigins};
use crate::handler::http::middleware::cors;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate dotenv;



pub mod application;
pub mod core;
pub mod infrastructure;
pub mod schema;
pub mod handler;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(cors::cors().to_cors().unwrap())
        .mount("/users/", handler::http::user_routes())
}
