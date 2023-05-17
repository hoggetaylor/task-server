use std::time::Duration;

use crate::{db::Conn, task::{Task, Type}};

pub async fn watch_tasks(conn: &Conn) {
    loop {
        tracing::info!("Polling tasks");
        if let Some(mut task) = Task::try_claim_task(conn).await {
            tracing::info!("Claimed task {}", task.id);
            handle_task(conn, &mut task).await
        } else {
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    }
}

async fn handle_task(conn: &Conn, task: &mut Task) {
    match task.task_type {
        Type::Foo => {
            tokio::time::sleep(Duration::from_secs(3)).await;
            tracing::info!("Foo {}", task.id);
        },
        Type::Bar => {
            match reqwest::get("https://www.whattimeisitrightnow.com/").await {
                Ok(res) => tracing::info!("Bar {}", res.status()),
                Err(e) => {
                    tracing::error!("Failed to make http request: {}", e);
                    task.reschedule(conn, chrono::Utc::now() + chrono::Duration::minutes(10)).await;
                    return
                }
            }
        },
        Type::Baz => {
            let n = (rand::random::<f64>() * 344.0).floor() as usize;
            tracing::info!("Baz {}", n);
        },
    }
    task.mark_complete(conn).await
}
