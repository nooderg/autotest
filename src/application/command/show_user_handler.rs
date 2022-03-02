use crate::core::domain::user::User;
use std::io::{Error, ErrorKind};

use crate::application::command::show_user_command::ShowUserCommand;
use crate::core::ports::user::UserRepository;
use crate::infrastructure::repository::user_repository::ORMUserRepository;


pub struct ShowUserCommandHandler {
    user_repository: ORMUserRepository,
}

impl ShowUserCommandHandler {
    pub fn new() -> ShowUserCommandHandler {
        ShowUserCommandHandler {
            user_repository: ORMUserRepository::new(),
        }
    }

    pub fn handle(&self, command: ShowUserCommand) -> Result<User, Error> {
        match self.user_repository.get(command.id().clone()) {
            Ok(t) => return Ok(t),
            Err(e) => return Err(Error::new(ErrorKind::BrokenPipe, e))
        };
    }
}