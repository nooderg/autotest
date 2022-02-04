use rocket::http::Status;
use rocket::serde::json::Json;

use crate::application::command::login_user_command::LoginUserCommand;
use crate::application::command::login_user_handler::LoginUserCommandHandler;
use crate::application::command::register_user_command::RegisterUserCommand;
use crate::application::command::register_user_handler::RegisterUserCommandHandler;
use crate::application::command::update_user_command::UpdateUserCommand;
use crate::application::command::update_user_handler::UpdateUserCommandHandler;

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
        data.password().clone(),
    );
    LoginUserCommandHandler::new().handle(command);
    Status::Ok
}

#[post("/update", format = "application/json", data = "<data>")]
pub fn update(data: Json<UpdateUserCommand>) -> Status {
    let command = UpdateUserCommand::new(
        data.id().clone(),
        data.first_name().clone(),
        data.last_name().clone(),
        data.email().clone(),
        data.password().clone(),
    );
    UpdateUserCommandHandler::new().handle(data.id().clone(),command);
    Status::Ok
}