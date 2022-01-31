use uuid::Uuid;
extern crate rocket;

use rocket::serde::Serialize;

#[derive(Serialize)]
pub struct User {
    pub username: String,
    pub email: String,
}

impl User {
    pub fn create_user() -> User {
        let user = User {
            username: Uuid::new_v4().to_string(),
            email: String::from(""),
        };
        return user
    }
}