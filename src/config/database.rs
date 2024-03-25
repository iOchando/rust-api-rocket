// use diesel::prelude::*;
// use diesel::sqlite::SqliteConnection;
// use dotenv::dotenv;

// pub struct DatabaseRepository {
//     pub connection: SqliteConnection,
// }

// impl DatabaseRepository {
//     pub fn new() -> Self {
//         dotenv().ok();
//         let database_url = "sqlite::memory:".to_string(); // Conexión con SQLite en memoria
//         let connection = SqliteConnection::establish(&database_url)
//             .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

//         DatabaseRepository { connection }
//     }
// }

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub struct DatabaseRepository {
    pub connection: Result<PgConnection, ConnectionError>,
}

impl DatabaseRepository {
    pub fn new() -> Self {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let mut connection = PgConnection::establish(&database_url);

        DatabaseRepository { connection }
    }
}
