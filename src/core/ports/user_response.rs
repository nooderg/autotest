use uuid::Uuid;
use std::time::SystemTime;

// TODO: use UserSummary in responses
pub struct UserSummary {
    id: Uuid,
    first_name: Option<String>,
    last_name: Option<String>,
    email: Option<String>,
    created_at: SystemTime,
}
