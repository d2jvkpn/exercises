use futures::future;
use tokio::time;

use std::{
    thread,
    time::{Duration, Instant},
};

async fn hello(input: i32) -> i32 {
    let id1 = thread::current().id();
    println!("--> hello: {input}, {id1:?}");
    time::sleep(Duration::from_secs(5)).await;

    let id2 = thread::current().id();
    println!("<-- hello: {input}, {id2:?}");

    input
    // id1 may not equals to id2
}

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    let now = Instant::now();
    let mut buffer = Vec::new();

    for i in 0..16 {
        // let handle = tokio::spawn(async move { hello(i).await });
        let handle = tokio::spawn(hello(i));
        buffer.push(handle);
    }

    thread::sleep(Duration::from_millis(10));
    println!("==> Joining");
    //	for i in buffer {
    //		let _ = i.await;
    //	}
    let results = future::join_all(buffer).await;
    println!("--> results: {results:?}, elapsed: {:.3?}", now.elapsed());
}
