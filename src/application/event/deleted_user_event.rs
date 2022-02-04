use eventbus::{Event};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct DeletedUserEvent {
    pub id: Uuid,
}

impl Event for DeletedUserEvent {
}
