use rocket::{get, catch};
use rocket::serde::json::{Value, json};

pub mod tasks_controller;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[catch(404)]
pub fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}