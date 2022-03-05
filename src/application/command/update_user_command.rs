use crate::core::services::user::hash_password;

#[derive(Serialize, Deserialize)]
pub struct UpdateUserCommand {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

impl UpdateUserCommand {
    pub fn new(first_name: String, last_name: String, email: String, password :String) -> UpdateUserCommand {
        UpdateUserCommand {
            first_name: first_name,
            last_name: last_name,
            email: email,
            password: hash_password(password),
        }
    }
}
