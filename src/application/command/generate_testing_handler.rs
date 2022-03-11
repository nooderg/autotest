use crate::application::command::generate_testing_command::GenerateTestingCommand;
use crate::core::domain::testing::Testing;
use std::env;


pub struct GenerateTestingCommandHandler {
}

impl GenerateTestingCommandHandler {
    pub fn new() -> GenerateTestingCommandHandler {
        GenerateTestingCommandHandler {
        }
    }

    pub fn handle(&self, command: GenerateTestingCommand, userid: String) -> Result<Testing, ureq::Error> {
        let ms_host = env::var("MICROSERVICE_HOST").expect("DATABASE_URL must be set");
        let ms_url = format!("http://{}:8080", ms_host);
        match ureq::post(&ms_url).send_json(ureq::json!({"file": command.file(), "user_id": userid})) {
            Ok(_t) => return Ok(Testing::new(command.file().to_string())),
            Err(e) => return Err(e)
      }
    }
}