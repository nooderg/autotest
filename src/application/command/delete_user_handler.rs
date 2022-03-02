use crate::application::command::delete_user_command::DeleteUserCommand;
use crate::core::ports::user::UserRepository;
use crate::infrastructure::repository::user_repository::ORMUserRepository;

pub struct DeleteUserCommandHandler {
    user_repository: ORMUserRepository,
}

impl DeleteUserCommandHandler {
    pub fn new() -> DeleteUserCommandHandler {
        DeleteUserCommandHandler {
            user_repository: ORMUserRepository::new(),
        }
    }

    pub fn handle(&self, command: DeleteUserCommand) -> bool {
        self.user_repository.delete(command.get_id())
    }
}