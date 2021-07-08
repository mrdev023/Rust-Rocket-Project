#[macro_use] extern crate diesel;

use rocket::*;
use rocket_sync_db_pools::{database};

mod task;

#[database("sqlite_logs")]
pub struct DbConn(diesel::SqliteConnection);

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/log/<id>")]
async fn get_log(conn: DbConn, id: i32) -> String {
    let result = task::Task::all(&conn).await;
    format!("test {}, {:?}", id, result)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, get_log]).attach(DbConn::fairing())
}
