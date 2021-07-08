#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_migrations;

use rocket::*;
use rocket::fairing::AdHoc;
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

async fn run_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    embed_migrations!();

    let conn = DbConn::get_one(&rocket).await.expect("database connection");
    conn.run(|c| embedded_migrations::run(c)).await.expect("can run migrations");

    rocket
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, get_log])
        .attach(DbConn::fairing())
        .attach(AdHoc::on_ignite("Run Migrations", run_migrations))
}
