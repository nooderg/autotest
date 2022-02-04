use diesel::RunQueryDsl;
use diesel::prelude::*;
use uuid::Uuid;

use crate::domain::user::User;
use crate::domain::user_repository::UserRepository;
use crate::infrastructure::models::write::new_user::NewUser;
use crate::infrastructure::repository::connection_manager::ConnectionManager;
use crate::schema::users;

pub struct ORMUserRepository {
    connection_manager: ConnectionManager,
}

impl UserRepository for ORMUserRepository {
    fn new() -> Self {
        ORMUserRepository {
            connection_manager: ConnectionManager::new(),
        }
    }

    fn add(&self, new_user: NewUser) -> User {
        let conn = &self.connection_manager.connection;
        diesel::insert_into(users::table)
            .values(&new_user)
            .get_result::<User>(conn)
            .expect("Error saving User")
    }

    fn show(&self, user_id: Uuid) -> User {
        use crate::schema::users::dsl::*;
        let conn = &self.connection_manager.connection;

        users.filter(id.eq(user_id))
            .first(conn)
            .expect("Error User not found")
    }
}