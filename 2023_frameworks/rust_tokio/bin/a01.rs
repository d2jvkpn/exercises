#![allow(dead_code)]

use tokio::{sync::mpsc, task::spawn_blocking};

use std::error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let (tx, mut rx) = mpsc::channel(2);
    let start = 5;

    let worker = spawn_blocking(move || {
        for x in 0..10 {
            // Stand in for complex computation
            tx.blocking_send(start + x).unwrap();
        }
    });

    let mut acc = 0;
    while let Some(v) = rx.recv().await {
        acc += v;
    }

    assert_eq!(acc, 95);
    worker.await.unwrap();

    Ok(())
}

async fn concat_str() -> Result<(), Box<dyn error::Error>> {
    let mut v = "Hello, ".to_string();

    let res = spawn_blocking(move || {
        // Stand-in for compute-heavy work or using synchronous APIs
        v.push_str("world");
        // Pass ownership of the value back to the asynchronous context
        v
    })
    .await?;

    // `res` is the value returned from the thread
    assert_eq!(res.as_str(), "Hello, world");

    Ok(())
}
