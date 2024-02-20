#![allow(dead_code)]

// use std::future::Future;
use tokio::{spawn, time};
use tokio_stream::StreamExt;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // println!("Hello, world!");

    // my_func().await;
    let func = my_func(42);

    println!("Let's Get Rusty!");
    func.await;

    // tasks: async and cpu-intensive
    let mut handles = vec![];

    for i in 0..8 {
        let handle = spawn(async move {
            my_func(i).await;
        });

        handles.push(handle);
    }

    handles.push(tokio::spawn(async {
        expensive_computation(1);
    }));

    handles.push(tokio::task::spawn_blocking(|| expensive_computation(2)));

    for handle in handles {
        let _ = handle.await;
    }

    // stream
    let mut stream =
        tokio_stream::iter(["Let's", "Get", "Rusty", "!"]).map(|v| v.to_ascii_uppercase());

    while let Some(v) = stream.next().await {
        println!("~~~ stream: {v}");
    }
}

async fn my_func(i: i32) {
    println!("~~~ {i}: I'm an async function!");

    let s1 = read_from_database().await;
    println!("~~~ {i}: First result: {s1:?}");

    let s2 = read_from_database().await;
    println!("~~~ {i}: Second result: {s2:?}");
}

async fn read_from_database() -> String {
    time::sleep(time::Duration::from_millis(50)).await;
    "DB Result".to_owned()
}

// CPU intensive blocking task
fn expensive_computation(id: u32) {
    println!("==> Start with expensive computation {id}");

    let mut ans = 0;
    for _ in 0..400_000_000 {
        ans += 1;
    }

    println!("==> Done with expensive computation {id}: {ans}");
}

/*
fn my_func() -> impl Future<Output = ()> {
    println!("I'm an async function!");
}

trait Future {
    type Output;
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
}

enum Poll<T> {
    Ready<T>,
    Pending,
}

*/
