#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate lazy_static;

pub mod application;
pub mod core;
pub mod infrastructure;
pub mod schema;
pub mod handler;


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/users/", handler::http::user_routes())
}
