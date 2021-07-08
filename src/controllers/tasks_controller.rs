use rocket::serde::json::Json;
use rocket::get;
use crate::models::{DbConn, task::Task};

#[get("/tasks/<id>")]
pub async fn show(conn: DbConn, id: i32) -> Option<Json<Task>> {
    Task::find_by(&conn, id).await.map(Json).ok()
}