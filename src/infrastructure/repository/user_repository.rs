use diesel::RunQueryDsl;
use diesel::prelude::*;
use uuid::Uuid;
use std::io::Error;

use crate::core::domain::user::User;
use crate::core::ports::user::UserRepository;
use crate::infrastructure::repository::connection_manager::ConnectionManager;
use crate::schema::users;
use crate::schema::users::dsl::*;

pub struct ORMUserRepository {
    connection_manager: ConnectionManager,
}

impl UserRepository for ORMUserRepository {
    fn new() -> Self {
        ORMUserRepository {
            connection_manager: ConnectionManager::new(),
        }
    }

    fn create(&self, new_user: User) -> Result<User, diesel::result::Error> {
        let conn = &self.connection_manager.connection;
        diesel::insert_into(users::table)
            .values(&new_user)
            .get_result::<User>(conn)
    }
   
    fn update(&self,  user_id:Uuid, new_user: User) -> Result<User, diesel::result::Error> {
        let conn = &self.connection_manager.connection;

        diesel::update(users::table.filter(id.eq(user_id)))
            .set((users::first_name.eq(new_user.first_name),
                users::last_name.eq(new_user.last_name),
                users::email.eq(new_user.email),
                users::password.eq(new_user.password),
                ))
            .get_result::<User>(conn)
    }
    
    fn get(&self, user_id: Uuid) -> Result<User, diesel::result::Error> {
        let conn = &self.connection_manager.connection;

        users.filter(id.eq(user_id))
            .first(conn)
    }
    
    fn get_by_email(&self, user_email: &String) -> Result<User, diesel::result::Error> {
        let conn = &self.connection_manager.connection;
        users.filter(email.eq(user_email))
            .first(conn)
    }
    
    fn delete(&self, user_id: Uuid) -> bool {
        let conn = &self.connection_manager.connection;
        let deleted_row = diesel::delete(users::table.filter(id.eq(user_id)))
            .execute(conn);
        
        deleted_row == Ok(1)
    }
}