use uuid::Uuid;
use std::time::SystemTime;

use crate::schema::users;

#[derive(Serialize, Deserialize, Queryable, AsChangeset, Insertable)]
#[table_name="users"]
pub struct User {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    #[serde(skip_serializing)] 
    pub password: String,
    pub created_at: SystemTime,
}

impl User {
    pub fn new(
        id: Uuid,
        first_name: String,
        last_name: String,
        email: String,
        password: String,
        created_at: SystemTime,
    ) -> Self {
        Self {
            id: id,
            first_name: first_name,
            last_name: last_name,
            email: email,
            password: password,
            created_at: created_at,
        }
    }
}
