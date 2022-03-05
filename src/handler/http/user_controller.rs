use rocket::http;
use rocket::serde::json::Json;
use rocket::response::status;
use uuid::Uuid;

use crate::core::domain::user::User;
use crate::core::ports::user_response::UserSummary;
use crate::handler::http::middleware::jwt;
use crate::application::command::show_user_command::ShowUserCommand;
use crate::application::command::show_user_handler::ShowUserCommandHandler;
use crate::application::command::login_user_command::LoginUserCommand;
use crate::application::command::login_user_handler::LoginUserCommandHandler;
use crate::application::command::register_user_command::RegisterUserCommand;
use crate::application::command::register_user_handler::RegisterUserCommandHandler;
use crate::application::command::update_user_command::UpdateUserCommand;
use crate::application::command::update_user_handler::UpdateUserCommandHandler;
use crate::application::command::delete_user_command::DeleteUserCommand;
use crate::application::command::delete_user_handler::DeleteUserCommandHandler;




#[post("/register", format = "application/json", data = "<data>")]
pub fn register(data: Json<RegisterUserCommand>) 
    -> Result<
        status::Custom<Json<UserSummary>>,
        status::Custom<Json<String>>> {

    let command = RegisterUserCommand::new(
        data.first_name().clone(),
        data.last_name().clone(),
        data.email().clone(),
        data.password().clone(),
    );

    match RegisterUserCommandHandler::new().handle(command) {
        Ok(t) => return Ok(
            status::Custom(
                http::Status::Ok, 
                Json(UserSummary::new(t))
            )),
        Err(e) => return Err(
            status::Custom(http::Status::BadRequest , Json(e.to_string())))
    }
}

#[post("/login", format = "application/json", data = "<data>")]
pub fn login(data: Json<LoginUserCommand>) 
    -> Result<
        status::Custom<Json<String>>,
        status::Custom<Json<String>>> {

    let command = LoginUserCommand::new(
        data.email().clone(),
        data.password().clone(),
    );

    match LoginUserCommandHandler::new().handle(command) {
        Ok(t) => return Ok(
            status::Custom(
                http::Status::Ok, 
                Json(t)
            )),
        Err(e) => return Err(
            status::Custom(http::Status::BadRequest , Json(e.to_string())))
    }
}


#[patch("/update", format = "application/json", data = "<data>")]
pub fn update(data: Json<UpdateUserCommand>, middleware: jwt::UserTokenClaims) 
    -> Result<
        status::Custom<Json<UserSummary>>,
        status::Custom<Json<String>>> {
            
    let command = UpdateUserCommand::new(
        data.first_name.clone(),
        data.last_name.clone(),
        data.email.clone(),
        data.password.clone(),
    );

    let id = uuid::Uuid::parse_str(&middleware.sub);
    match UpdateUserCommandHandler::new().handle(id.unwrap(), command) {
        Ok(t) => Ok(status::Custom(
            http::Status::Ok,
            Json(UserSummary::new(t))
        )),
        Err(e) => Err(status::Custom(
            http::Status::BadRequest,
            Json(e.to_string())
        ))
    }
}

#[get("/<uuid>", format = "application/json")]
pub fn show(uuid: Uuid, middleware: jwt::UserTokenClaims) 
    -> Result<
        status::Custom<Json<UserSummary>>,
        status::Custom<Json<String>>> {
        
    let command = ShowUserCommand::new(
        uuid.clone(),
    );
    
    match ShowUserCommandHandler::new().handle(command) {
        Ok(t) => Ok(status::Custom(
            http::Status::Ok,
            Json(UserSummary::new(t))
        )),
        Err(e) => Err(status::Custom(
            http::Status::BadRequest,
            Json(e.to_string())
        ))
    }
}

#[delete("/", format = "application/json")]
pub fn delete(middleware: jwt::UserTokenClaims) -> http::Status {
    let id = uuid::Uuid::parse_str(&middleware.sub).unwrap();
    let command = DeleteUserCommand::new(id);
    
    if DeleteUserCommandHandler::new().handle(command) {
        return http::Status::NoContent;
    };
    http::Status::BadRequest
}
