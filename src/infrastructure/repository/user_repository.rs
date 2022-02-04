use diesel::RunQueryDsl;
use diesel::prelude::*;
use crate::domain::user::User;
use crate::domain::user_repository::UserRepository;
use crate::infrastructure::models::write::new_user::NewUser;
use crate::infrastructure::repository::connection_manager::ConnectionManager;
use crate::schema::users;
use crate::schema::users::dsl::*;
use uuid::Uuid;

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

    fn remove(&self, user_id: Uuid) -> bool {
        let conn = &self.connection_manager.connection;
        diesel::delete(users::table.filter(id.eq(user_id))).execute(conn);
        return true;
    }
}