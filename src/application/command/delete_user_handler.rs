use crate::application::command::delete_user_command::DeleteUserCommand;
use crate::domain::user_repository::UserRepository;
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
        let user_removed = self.user_repository.remove(command.get_id());
        return user_removed;
    }
}