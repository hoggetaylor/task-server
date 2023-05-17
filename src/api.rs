use axum::{
    routing::get,
    Json, Router, extract::{Path, Query, State}, http::StatusCode, response::{Response, IntoResponse}
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
    println!("Listening for requests on {}", addr);
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

async fn create_task(State(state): State<AppState>, Json(create): Json<CreateTask>) -> Response {
    let res = Task::create(&state.conn, create).await;
    match res {
        Ok(task) => (StatusCode::OK, Json(task)).into_response(),
        Err(e) => {
            eprintln!("Error creating task: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json("Internal Server Error")).into_response()
        }
    }
}

async fn get_task(State(state): State<AppState>, Path(tid): Path<String>) -> Response {
    let uuid = Uuid::parse_str(&tid);
    if uuid.is_err() {
        return (StatusCode::NOT_FOUND, Json("Not Found")).into_response()
    }
    let res = Task::get(&state.conn, &uuid.unwrap()).await;
    match res {
        Ok(task) => (StatusCode::OK, Json(task)).into_response(),
        Err(e) => {
            eprintln!("Error creating task: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json("Internal Server Error")).into_response()
        }
    }
}

async fn delete_task(State(state): State<AppState>, Path(tid): Path<String>) -> Response {
    let uuid = Uuid::parse_str(&tid);
    if uuid.is_err() {
        return (StatusCode::NOT_FOUND, Json("Not Found")).into_response()
    }
    let res = Task::delete(&state.conn, &uuid.unwrap()).await;
    match res {
        Ok(task) => (StatusCode::OK, Json(task)).into_response(),
        Err(e) => {
            eprintln!("Error creating task: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json("Internal Server Error")).into_response()
        }
    }
}

async fn list_tasks(State(state): State<AppState>, Query(q): Query<ListTasks>) -> Response {
    let res = Task::list(&state.conn, &q).await;
    match res {
        Ok(tasks) => (StatusCode::OK, Json(tasks)).into_response(),
        Err(e) => {
            eprintln!("Error creating task: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json("Internal Server Error")).into_response()
        }
    }
}
