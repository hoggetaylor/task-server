use std::time::Duration;

use crate::{db::Conn, task::{Task, Type}};

pub async fn watch_tasks(conn: &Conn) {
    loop {
        println!("Polling tasks");
        let res = Task::try_claim_task(conn).await;
        match res {
            Err(e) => {
                eprintln!("Failed to poll for tasks: {}", e);
                tokio::time::sleep(Duration::from_secs(5)).await;
            },
            Ok(opt) => {
                if let Some(mut task) = opt {
                    println!("Claimed task {}", task.id);
                    let res = handle_task(conn, &mut task).await;
                    if let Err(e) = res {
                        eprintln!("Error handling task: ${}", e);
                    }
                } else {
                    tokio::time::sleep(Duration::from_secs(5)).await;
                }
            }
        }
    }
}

async fn handle_task(conn: &Conn, task: &mut Task) -> Result<(), sqlx::Error> {
    match task.task_type {
        Type::Foo => {
            tokio::time::sleep(Duration::from_secs(3)).await;
            println!("Foo {}", task.id);
        },
        Type::Bar => {
            match reqwest::get("https://www.whattimeisitrightnow.com/").await {
                Ok(res) => println!("Bar {}", res.status()),
                Err(e) => {
                    eprintln!("Failed to make http request: {}", e);
                    task.reschedule(conn, chrono::Utc::now() + chrono::Duration::minutes(10)).await?;
                    return Ok(())
                }
            }
        },
        Type::Baz => {
            let n = (rand::random::<f64>() * 344.0).floor() as usize;
            println!("Baz {}", n);
        },
    }
    task.mark_complete(conn).await
}
