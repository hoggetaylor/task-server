use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize, Serializer};
use sqlx::{types::Uuid, QueryBuilder};

use crate::db::Conn;

#[derive(Debug, Deserialize)]
pub struct ListTasks {
    pub page_size: Option<usize>,
    pub page: Option<usize>,
    pub task_type: Option<Type>,
    pub task_state: Option<State>
}

#[derive(Debug, Deserialize)]
pub struct CreateTask {
    pub execution_time: DateTime<Utc>,
    pub task_type: Type
}

// Apparently the "serde" feature of the sqlx crate is supposed to provide
// this functionality but cargo fails to find an appropriate version for
// the crate when the feature is enabled. This works for now.
fn uuid_serialize<S: Serializer>(u: &Uuid, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_str(&u.as_hyphenated().to_string())
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Task {
    #[serde(serialize_with="uuid_serialize")]
    pub id: Uuid,
    pub execution_time: DateTime<Utc>,
    pub task_type: Type,
    pub task_state: State
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name="task_type")]
pub enum Type {
    Foo,
    Bar,
    Baz
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name="task_state")]
pub enum State {
    Scheduled,
    Running,
    Completed
}

impl Task {

    pub async fn try_claim_task(conn: &Conn) -> Result<Option<Task>, sqlx::Error> {
        sqlx::query_as::<_, Task>(
            "UPDATE tasks SET task_state='Running' WHERE id = (
                SELECT id FROM tasks WHERE
                    task_state='Scheduled' AND
                    execution_time <= now()
                ORDER BY execution_time
                LIMIT 1
            ) RETURNING *"
        ).fetch_optional(conn).await
    }

    pub async fn mark_complete(&mut self, conn: &Conn) -> Result<(), sqlx::Error> {
        let res = sqlx::query("UPDATE tasks SET task_state='Completed' WHERE id = $1")
            .bind(self.id)
            .execute(conn).await;
        if res.is_ok() {
            self.task_state = State::Completed;
        }
        res.map(|_n| ())
    }

    pub async fn reschedule(&mut self, conn: &Conn, when: chrono::DateTime<Utc>) -> Result<(), sqlx::Error> {
        let res = sqlx::query("UPDATE tasks SET task_state='Requested', execution_time=$1 WHERE id = $2")
            .bind(when)
            .bind(self.id)
            .execute(conn).await;
        if res.is_ok() {
            self.task_state = State::Scheduled;
            self.execution_time = when;
        }
        res.map(|_n| ())
    }

    pub async fn create(conn: &Conn, create: CreateTask) -> Result<Task, sqlx::Error> {
        sqlx::query_as::<_, Task>("INSERT INTO tasks (execution_time, task_type) VALUES ($1, $2) RETURNING *")
            .bind(create.execution_time)
            .bind(create.task_type)
            .fetch_one(conn).await
    }

    pub async fn get(conn: &Conn, id: &Uuid) -> Result<Task, sqlx::Error> {
        sqlx::query_as::<_, Task>("SELECT * FROM tasks WHERE id = $1")
            .bind(id)
            .fetch_one(conn).await
    }

    pub async fn delete(conn: &Conn, id: &Uuid) -> Result<Task, sqlx::Error> {
        sqlx::query_as::<_, Task>("DELETE FROM tasks WHERE id = $1 RETURNING *")
            .bind(id)
            .fetch_one(conn).await
    }

    pub async fn list(conn: &Conn, query: &ListTasks) -> Result<Vec<Task>, sqlx::Error> {
        let page_size = query.page_size.unwrap_or(10);
        let offset = query.page.unwrap_or(0) * page_size;
        let mut builder = QueryBuilder::new("SELECT * FROM tasks ");
        if query.task_type.is_some() || query.task_state.is_some() {
            builder.push(" WHERE ");
        }
        if let Some(task_type) = &query.task_type {
            builder.push(" task_type = ")
                   .push_bind(task_type);
        }
        if let Some(task_state) = &query.task_state {
            if query.task_type.is_some() {
                builder.push(" AND ");
            }
            builder.push(" task_state = ")
                   .push_bind(task_state);
        }
        builder.push(" ORDER BY execution_time ")
               .push(" LIMIT ")
               .push_bind(page_size as i64)
               .push(" OFFSET ")
               .push_bind(offset as i64);
        builder.build_query_as().fetch_all(conn).await
    }

}
