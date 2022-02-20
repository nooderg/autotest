use crate::services::password_service::hash_password;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct UpdateUserCommand {
    id: Uuid,
    first_name: String,
    last_name: String,
    email: String,
    password: String,
}

impl UpdateUserCommand {
    pub fn new(id: Uuid, first_name: String, last_name: String, email: String, password :String) -> UpdateUserCommand {
        UpdateUserCommand {
            id: id,
            first_name: first_name,
            last_name: last_name,
            email: email,
            password : hash_password(password),
        }
    }
    pub fn id(&self) -> Uuid {
        return self.id;
    }
    
    pub fn first_name(&self) -> &String {
        &self.first_name
    }

    pub fn last_name(&self) -> &String {
        &self.last_name
    }

    pub fn email(&self) -> &String {
        &self.email
    }

    pub fn password(&self) -> &String {
        &self.password
    }
}
