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
    pub file_url: String,
    pub created_at: SystemTime,
}

impl User {
    pub fn new(
        id: Uuid,
        first_name: String,
        last_name: String,
        email: String,
        password: String,
        file_url: String,
        created_at: SystemTime,
    ) -> Self {
        Self {
            id: id,
            first_name: first_name,
            last_name: last_name,
            email: email,
            password: password,
            file_url: file_url,
            created_at: created_at,
        }
    }
}
