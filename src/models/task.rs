use rocket::serde::Serialize;
use diesel::{self, result::QueryResult, Queryable, Insertable, prelude::*};

table! {
    tasks {
        id -> Nullable<Integer>,
        description -> Text,
        completed -> Bool,
    }
}

use tasks::dsl::{tasks as all_tasks};

use crate::DbConn;

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
        conn.run(|c| {
            all_tasks.order(tasks::id.desc()).load::<Task>(c)
        }).await
    }
}