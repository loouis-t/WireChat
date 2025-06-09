use diesel::sqlite::SqliteConnection;
use diesel::r2d2::{ConnectionManager, Pool};


pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;

pub fn init<P: AsRef<std::path::Path>>(db_path: P) -> DbPool {
    let db_path = db_path.as_ref().to_string_lossy().to_string();
    let manager = ConnectionManager::<SqliteConnection>::new(db_path);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}
