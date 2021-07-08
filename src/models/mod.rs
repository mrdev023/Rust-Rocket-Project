use rocket_sync_db_pools::database;
use diesel::SqliteConnection;

#[database("sqlite_logs")]
pub struct DbConn(SqliteConnection);

pub mod task;