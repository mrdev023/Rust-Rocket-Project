use rocket::serde::json::Json;
use rocket::{get, post, patch, delete};
use crate::models::{DbConn, task::Task};

#[get("/tasks")]
pub async fn index(conn: DbConn) -> Option<Json<Vec<Task>>> {
    Task::all(&conn).await.map(Json).ok()
}

#[get("/tasks/<id>")]
pub async fn show(conn: DbConn, id: i32) -> Option<Json<Task>> {
    Task::find_by(&conn, id).await.map(Json).ok()
}

#[post("/tasks", data = "<task>")]
pub async fn create(conn: DbConn, task: Json<Task>) -> Option<Json<usize>> {
    Task::create(conn, task.clone()).await.map(Json).ok()
}

#[patch("/tasks/<id>", data = "<task>")]
pub async fn update(conn: DbConn, id: i32, task: Json<Task>) -> Option<Json<usize>> {
    Task::update(conn, id, task.clone()).await.map(Json).ok()
}

#[delete("/tasks/<id>")]
pub async fn delete(conn: DbConn, id: i32) -> Option<Json<usize>> {
    Task::delete(conn, id).await.map(Json).ok()
}