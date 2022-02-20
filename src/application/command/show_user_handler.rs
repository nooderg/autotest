use crate::domain::user::User;

use crate::application::command::show_user_command::ShowUserCommand;
use crate::domain::user_repository::UserRepository;
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

    pub fn handle(&self, command: ShowUserCommand) -> User {
        self.user_repository.show(command.id().clone())
    }
}