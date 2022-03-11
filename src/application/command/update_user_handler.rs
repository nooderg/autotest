use crate::application::command::update_user_command::UpdateUserCommand;
use crate::core::domain;
use crate::core::ports::user::UserRepository;
use crate::core::domain::user::User;
use crate::infrastructure::repository::user_repository::ORMUserRepository;
use uuid::Uuid;
use std::io::{Error, ErrorKind};

pub struct UpdateUserCommandHandler {
    user_repository: ORMUserRepository,
}

impl UpdateUserCommandHandler {
    pub fn new() -> UpdateUserCommandHandler {
        UpdateUserCommandHandler {
            user_repository: ORMUserRepository::new(),
        }
    }

    pub fn handle(&self, id:Uuid, command: UpdateUserCommand) -> Result<domain::user::User, Error> {
        let user = User {
            id: id.clone(),
            first_name: command.first_name.clone(),
            last_name: command.last_name.clone(),
            email: command.email.clone(),
            password: command.password.clone(),
            file_url: "".to_owned(),
            created_at: std::time::SystemTime::now(),
        };
        match self.user_repository.update(id.clone(), user) {
            Ok(t) => return Ok(t),
            Err(e) => return Err(Error::new(ErrorKind::BrokenPipe, e))
        };
    }
}
