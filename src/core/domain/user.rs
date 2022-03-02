use uuid::Uuid;
use std::time::SystemTime;

use crate::schema::users;

#[derive(Serialize, Deserialize, Queryable, AsChangeset, Insertable)]
#[table_name="users"]
pub struct User {
    pub id: Uuid,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    #[serde(skip_serializing)] 
    pub password: Option<String>,
    pub created_at: SystemTime,
}

impl User {
    pub fn new(
        id: Uuid,
        first_name: Option<String>,
        last_name: Option<String>,
        email: Option<String>,
        password: Option<String>,
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
