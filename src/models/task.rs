use rocket::serde::Serialize;
use diesel::{self, result::QueryResult, Queryable, Insertable, prelude::*};
use super::DbConn;

table! {
    tasks (id) {
        id -> Nullable<Integer>,
        description -> Text,
        completed -> Bool,
    }
}

#[derive(Serialize, Queryable, Insertable, Debug, Clone)]
#[serde(crate = "rocket::serde")]
#[table_name="tasks"]
pub struct Task {
    pub id: Option<i32>,
    pub description: String,
    pub completed: bool
}

impl Task {
    pub async fn all(conn: &DbConn) -> QueryResult<Vec<Task>> {
        conn.run(move |c| {
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
}