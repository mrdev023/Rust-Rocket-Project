#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_migrations;

mod models;
mod controllers;

use rocket::*;
use rocket::fairing::AdHoc;
use models::DbConn;
use rocket_dyn_templates::Template;

async fn run_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    embed_migrations!();

    let conn = DbConn::get_one(&rocket).await.expect("database connection");
    conn.run(|c| embedded_migrations::run(c)).await.expect("can run migrations");

    rocket
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![
            controllers::tasks_controller::index_template
        ])
        .mount("/api/", routes![
            controllers::index,
            controllers::tasks_controller::index,
            controllers::tasks_controller::show,
            controllers::tasks_controller::create,
            controllers::tasks_controller::update,
            controllers::tasks_controller::delete
        ])
        .register("/", catchers![
            controllers::not_found
        ])
        .attach(DbConn::fairing())
        .attach(Template::fairing())
        .attach(AdHoc::on_ignite("Run Migrations", run_migrations))
}
