use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

pub fn init() -> SqliteConnection {
    let database_url = "db.sqlite";
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
