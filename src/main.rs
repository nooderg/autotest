#[macro_use] extern crate rocket;

mod models;

use rocket::serde::json::Json;
use models::users::User;

#[get("/")]
fn hello() -> Json<User> {
    let user = User::create_user();
    return Json(user)
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![hello])
}