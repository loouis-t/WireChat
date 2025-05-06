use diesel::sqlite::SqliteConnection;
use diesel::r2d2::{ConnectionManager, Pool};

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;

pub fn init() -> DbPool {
    // Tente de lire DATABASE_URL, sinon utilise "./db.sqlite"
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "./db.sqlite".to_string());

    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}
