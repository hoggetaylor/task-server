use axum::{
    routing::get,
    Json, Router, extract::{Path, Query, State}, http::StatusCode
};
use sqlx::types::Uuid;
use std::{net::SocketAddr, sync::Arc};

use crate::{task::{ListTasks, CreateTask, Task}, db::Conn};

#[derive(Clone)]
struct AppState {
    conn: Arc<Conn>
}

pub async fn listen_on_port(conn: Arc<Conn>, port: u16) {
    let app = setup_routes(conn);
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    tracing::info!("Listening for requests on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn setup_routes(conn: Arc<Conn>) -> Router {
    Router::new()
        .route("/", get(list_tasks).post(create_task))
        .route("/:tid", get(get_task).delete(delete_task))
        .with_state(AppState { conn })
}

async fn create_task(State(state): State<AppState>, Json(create): Json<CreateTask>) -> (StatusCode, Json<Task>) {
    let task = Task::create(&state.conn, create).await;
    (StatusCode::OK, Json(task))
}

async fn get_task(State(state): State<AppState>, Path(tid): Path<String>) -> (StatusCode, Json<Task>) {
    let uuid = Uuid::parse_str(&tid).expect("Invalid identifier");
    let task = Task::get(&state.conn, &uuid).await;
    (StatusCode::OK, Json(task))
}

async fn delete_task(State(state): State<AppState>, Path(tid): Path<String>) -> (StatusCode, Json<Task>) {
    let uuid = Uuid::parse_str(&tid).expect("Invalid identifier");
    let task = Task::delete(&state.conn, &uuid).await;
    (StatusCode::OK, Json(task))
}

async fn list_tasks(State(state): State<AppState>, Query(q): Query<ListTasks>) -> (StatusCode, Json<Vec<Task>>) {
    let tasks = Task::list(&state.conn, &q).await;
    (StatusCode::OK, Json(tasks))
}
