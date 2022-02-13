use crate::application::command::login_user_command::LoginUserCommand;
use crate::domain::user_repository::UserRepository;
use crate::infrastructure::repository::user_repository::ORMUserRepository;

pub struct LoginUserCommandHandler {
    user_repository: ORMUserRepository,
}

impl LoginUserCommandHandler {
    pub fn new() -> LoginUserCommandHandler {
        LoginUserCommandHandler {
            user_repository: ORMUserRepository::new(),
        }
    }

    pub fn handle(&self, command: LoginUserCommand) -> () {
        //TODO
    }
}