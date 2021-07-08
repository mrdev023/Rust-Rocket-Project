use rocket::serde::{Serialize, Deserialize};
use diesel::{self, result::QueryResult, Queryable, Insertable, AsChangeset, prelude::*};
use super::DbConn;

table! {
    tasks (id) {
        id -> Nullable<Integer>,
        description -> Text,
        completed -> Bool,
    }
}

#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset, Debug, Clone)]
#[serde(crate = "rocket::serde")]
#[table_name="tasks"]
pub struct Task {
    pub id: Option<i32>,
    pub description: String,
    pub completed: bool
}

impl Task {
    pub async fn all(conn: &DbConn) -> QueryResult<Vec<Task>> {
        conn.run(|c| {
            tasks::table.load(c)
        }).await
    }

    pub async fn find_by(conn: &DbConn, id: i32) -> QueryResult<Task> {
        conn.run(move |c| {
            tasks::table
                .filter(tasks::id.eq(id))
                .first(c)
        }).await
    }

    pub async fn create(db: DbConn, task: Task) -> QueryResult<usize> {
        db.run(move |conn| {
            diesel::insert_into(tasks::table)
                .values(&task)
                .execute(conn)
        }).await
    }

    pub async fn update(db: DbConn, id: i32, task: Task) -> QueryResult<usize> {
        db.run(move |conn| {
            diesel::update(tasks::table)
                .set(&task)
                .filter(tasks::id.eq(id))
                .execute(conn)
        }).await
    }

    pub async fn delete(db: DbConn, id: i32) -> QueryResult<usize> {
        db.run(move |conn| {
            diesel::delete(tasks::table)
                .filter(tasks::id.eq(id))
                .execute(conn)
        }).await
    }
}