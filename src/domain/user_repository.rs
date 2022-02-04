use crate::infrastructure::models::write::new_user::NewUser;
use crate::domain::user::User;
use uuid::Uuid;

pub trait UserRepository {
    fn new() -> Self;
    fn add(&self, new_user: NewUser) -> User;
    fn update(&self,id:Uuid, new_user: NewUser) -> User;
}
