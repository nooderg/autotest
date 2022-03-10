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
        /*let mut map = HashMap::new();
        map.insert("file", command.file());
        let client = reqwest::blocking::Client::new();
        match client.post("www.google.com").json(&map).send() {
            Ok(_t) => return Ok(Testing::new(command.file().to_string())),
            Err(e) => return Err(Error::new(ErrorKind::BrokenPipe, e))
        };*/
        match ureq::post("http://www.google.com").send_json(ureq::json!({"file": command.file()})) {
            Ok(_t) => return Ok(Testing::new(command.file().to_string())),
            Err(e) => return Err(e)
      }
    }
}