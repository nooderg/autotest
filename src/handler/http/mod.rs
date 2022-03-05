pub mod user_controller;
pub mod middleware;

pub fn user_routes() -> Vec<rocket::Route> {
    routes![
        user_controller::register,
        user_controller::login,
        user_controller::update,
        user_controller::show,
        user_controller::delete,
    ]
}