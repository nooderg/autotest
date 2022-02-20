use crate::application::command::update_user_command::UpdateUserCommand;
use crate::domain::user_repository::UserRepository;
use crate::infrastructure::models::write::new_user::NewUser;
use crate::infrastructure::repository::user_repository::ORMUserRepository;
use uuid::Uuid;

pub struct UpdateUserCommandHandler {
    user_repository: ORMUserRepository,
}

impl UpdateUserCommandHandler {
    pub fn new() -> UpdateUserCommandHandler {
        UpdateUserCommandHandler {
            user_repository: ORMUserRepository::new(),
        }
    }

    pub fn handle(&self, id:Uuid,command: UpdateUserCommand) -> () {
        let user = NewUser {
            first_name: command.first_name().clone(),
            last_name: command.last_name().clone(),
            email: command.email().clone(),
            password: command.email().clone(),
        };
        let update_user = self.user_repository.update(command.id().clone() ,user);
        update_user.update_user();
    }
}
