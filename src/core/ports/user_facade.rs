use crate::core::domain::user::User;

#[derive(Serialize, Deserialize)]
pub struct UserFacade {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

impl UserFacade {
    pub fn new(user: User) -> UserFacade {
        UserFacade{
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
            password: user.password,
        }
    }
}