#![allow(dead_code)]

// use std::future::Future;
use tokio::{spawn, time};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // println!("Hello, world!");

    // my_func().await;
    let func = my_func(42);

    println!("Let's Get Rusty!");
    func.await;

    // tasks
    let mut handles = vec![];

    for i in 0..8 {
        let handle = spawn(async move {
            my_func(i).await;
        });

        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.await;
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
