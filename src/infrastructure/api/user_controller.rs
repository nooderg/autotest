use rocket::http::Status;
use rocket::serde::json::Json;

use crate::application::command::register_user_command::RegisterUserCommand;
use crate::application::command::register_user_handler::RegisterUserCommandHandler;

#[post("/register", format = "application/json", data = "<data>")]
pub fn register(data: Json<RegisterUserCommand>) -> Status {
    let command = RegisterUserCommand::new(
        data.first_name().clone(),
        data.last_name().clone(),
        data.email().clone(),
        data.password().clone(),
    );
    RegisterUserCommandHandler::new().handle(command);
    Status::Ok
}

#[post("/login", format = "application/json", data = "<data>")]
pub fn login(data: Json<LoginUserCommand>) -> Status {
    let command = LoginUserCommand::new(
        data.email().clone(),
    );
    Status::Ok
}
