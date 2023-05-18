use std::sync::Arc;

mod task;
mod api;
mod db;
mod worker;

#[tokio::main]
async fn main() {
    let conn = Arc::new(
        db::connect("postgres://taskadmin:taskadmin@localhost/task-server").await
    );
    db::run_migrations(&conn).await;
    tokio::join!(
        api::listen_on_port(conn.clone(), 3000),
        worker::watch_tasks(&conn)
    );
}
