use rocket::get;

pub mod tasks_controller;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}