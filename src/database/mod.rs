pub mod db_setup;
pub mod peer;
pub mod message;
pub mod schema;

use diesel::r2d2::{ConnectionManager, Pool};
use diesel::SqliteConnection;

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;

pub fn establish_pool(database_url: &str) -> DbPool {
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}
