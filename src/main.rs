use std::sync::Arc;


mod task;
mod api;
mod db;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let conn = Arc::new(
        db::connect("postgres://taskadmin:taskadmin@localhost/task-server").await
    );
    db::run_migrations(&conn).await;
    api::listen_on_port(conn, 3000).await
}
