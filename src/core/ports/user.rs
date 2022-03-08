use uuid::Uuid;

use crate::core::ports::user_facade::UserFacade;
use crate::core::domain::user::User;


// UserRepository is the Interface to interact with user storage
pub trait UserRepository {
    fn new() -> Self;
    fn create(&self, new_user: User) -> Result<User, diesel::result::Error>;
    fn update(&self,id:Uuid, new_user: User) -> Result<User, diesel::result::Error>;
    fn get(&self, id: Uuid) -> Result<User, diesel::result::Error>;
    fn get_by_email(&self, email: &String) -> Result<User, diesel::result::Error>;
    fn delete(&self, id: Uuid) -> bool;
}

// UserService is the interface and entrypoint to acces domain logic
// TODO: impletement the interface
pub trait UserService {
    fn register(new_user: UserFacade) -> User;
    fn login();
    fn verify_token();
    fn check_permissions();
}