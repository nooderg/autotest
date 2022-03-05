use uuid::Uuid;
use std::time::SystemTime;
use crate::core::domain::user::User;

#[derive(Serialize, Deserialize)]
pub struct UserSummary {
    id: Uuid,
    first_name: String,
    last_name: String,
    email: String,
    created_at: SystemTime,
}

impl UserSummary {
    pub fn new(user: User) -> UserSummary {
        UserSummary{
            id: user.id,
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
            created_at: user.created_at
        }
    }
}