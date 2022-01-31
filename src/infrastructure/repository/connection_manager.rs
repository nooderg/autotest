use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub struct ConnectionManager {
    pub connection: PgConnection,
}

impl ConnectionManager {
    pub fn new() -> Self {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        
        ConnectionManager {
            connection: PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url)),
        }
    }
}