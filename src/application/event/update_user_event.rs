use eventbus::{Event};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct UpdateUserEvent {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

impl Event for UpdateUserEvent {
}
