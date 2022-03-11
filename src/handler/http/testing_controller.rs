use rocket::http;
use rocket::serde::json::Json;
use rocket::response::status;
use crate::core::ports::testing_response::TestingSummary;
use crate::application::command::generate_testing_command::GenerateTestingCommand;
use crate::application::command::generate_testing_handler::GenerateTestingCommandHandler;
use crate::handler::http::middleware::jwt;


#[post("/generate", format = "application/json", data = "<data>")]
pub fn generate(data: Json<GenerateTestingCommand>, middleware: jwt::UserTokenClaims) 
    -> Result<
        status::Custom<Json<TestingSummary>>,
        status::Custom<Json<String>>> {

    let command = GenerateTestingCommand::new(
        data.file().clone()
    );

    let id = uuid::Uuid::parse_str(&middleware.sub);

    match GenerateTestingCommandHandler::new().handle(command, id.unwrap().to_string()) {
        Ok(t) => return Ok(status::Custom(http::Status::Ok, Json(TestingSummary::new(t)))),
        Err(e) => return Err(status::Custom(http::Status::BadRequest , Json(e.to_string())))
    }
}