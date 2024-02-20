#![allow(dead_code)]

use tokio::{spawn, sync::mpsc, task::spawn_blocking, time::sleep};

use std::{
    error, thread,
    time::{Duration, Instant},
};

#[tokio::main(flavor = "multi_thread", worker_threads = 1)]
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

    buffer().await;

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

async fn buffer() {
    let now = Instant::now();
    let mut buffer = Vec::new();

    for i in 0..16 {
        // let handle = tokio::spawn(async move { hello(i).await });
        let handle = spawn(hello(i));
        buffer.push(handle);
    }

    futures::future::join_all(buffer).await;
    println!("<<< elapsed: {:.2?}", now.elapsed());
}

async fn hello(val: usize) {
    println!("==> {val}, {:?}", thread::current().id());
    log_mdc::insert("hello", val.to_string());
    sleep(Duration::new(2, 0)).await;

    let mut mdc = vec![];
    log_mdc::iter(|k, v| mdc.push((k.to_owned(), v.to_owned())));

    println!("<== {val}, {:?}, {:?}", thread::current().id(), mdc);
}
