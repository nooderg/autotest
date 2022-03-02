use std::io::{Error, ErrorKind};
use pwhash::bcrypt;

use crate::application::command::login_user_command::LoginUserCommand;
use crate::core::ports::user::UserRepository;
use crate::infrastructure::repository::user_repository::ORMUserRepository;
use crate::core::services;

pub struct LoginUserCommandHandler {
    user_repository: ORMUserRepository,
}


impl LoginUserCommandHandler {
    pub fn new() -> LoginUserCommandHandler {
        LoginUserCommandHandler {
            user_repository: ORMUserRepository::new(),
        }
    }

    pub fn handle(&self, command: LoginUserCommand) -> Result<String, Error> {
        let user = match self.user_repository.get_by_email(command.email()) {
            Ok(t) => t,
            Err(e) => return Err(Error::new(ErrorKind::BrokenPipe, e))
        };
        let h = user.password.unwrap();
        let p = command.password();
 
        if !bcrypt::verify(p, &h) {
            return Err(Error::new(ErrorKind::InvalidInput, "passwords dont' match"));
        };

        services::user::create_token(user.id)
    }
}