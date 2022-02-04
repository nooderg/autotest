use rocket::http::Status;
use rocket::serde::json::Json;

use crate::application::command::login_user_command::LoginUserCommand;
use crate::application::command::login_user_handler::LoginUserCommandHandler;
use crate::application::command::register_user_command::RegisterUserCommand;
use crate::application::command::register_user_handler::RegisterUserCommandHandler;
use crate::application::command::delete_user_command::DeleteUserCommand;
use crate::application::command::delete_user_handler::DeleteUserCommandHandler;

#[post("/register", format = "application/json", data = "<data>")]
pub fn register(data: Json<RegisterUserCommand>) -> Status {
    let command = RegisterUserCommand::new(
        data.first_name().clone(),
        data.last_name().clone(),
        data.email().clone(),
        data.password().clone(),
    );
    RegisterUserCommandHandler::new().handle(command);
    Status::Created
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

#[delete("/", format = "application/json", data = "<data>")]
pub fn delete(data: Json<DeleteUserCommand>) -> Status {
    let command = DeleteUserCommand::new(
        data.get_id().clone(),
    );
    
    if DeleteUserCommandHandler::new().handle(command) {
        return Status::NoContent;
    };
    Status::BadRequest
}