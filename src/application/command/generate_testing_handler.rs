use crate::application::command::generate_testing_command::GenerateTestingCommand;
use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use crate::core::domain::testing::Testing;

pub struct GenerateTestingCommandHandler {
}

impl GenerateTestingCommandHandler {
    pub fn new() -> GenerateTestingCommandHandler {
        GenerateTestingCommandHandler {
        }
    }

    pub fn handle(&self, command: GenerateTestingCommand) -> Result<Testing, ureq::Error> {
        match ureq::post("http://www.google.com").send_json(ureq::json!({"file": command.file()})) {
            Ok(_t) => return Ok(Testing::new(command.file().to_string())),
            Err(e) => return Err(e)
      }
    }
}