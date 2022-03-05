use crate::application::command::register_user_command::RegisterUserCommand;
use crate::core::ports::user::UserRepository;
use crate::core::domain::user::User;
use crate::infrastructure::repository::user_repository::ORMUserRepository;
use uuid::Uuid;
use std::time::SystemTime;
use std::io::{Error, ErrorKind};

pub struct RegisterUserCommandHandler {
    user_repository: ORMUserRepository,
}

impl RegisterUserCommandHandler {
    pub fn new() -> RegisterUserCommandHandler {
        RegisterUserCommandHandler {
            user_repository: ORMUserRepository::new(),
        }
    }

    pub fn handle(&self, command: RegisterUserCommand) -> Result<User, Error> {
        let user = User {
            id: Uuid::new_v4(),
            first_name: command.first_name().clone(),
            last_name: command.last_name().clone(),
            email: command.email().clone(),
            password: command.password().clone(),
            created_at: SystemTime::now(),
        };
        
        match self.user_repository.create(user) {
            Ok(t) => return Ok(t),
            Err(e) => return Err(Error::new(ErrorKind::BrokenPipe, e))
        };
    }
}
