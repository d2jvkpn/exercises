use std::{
    thread,
    time::{Duration, Instant},
};
use tokio::time::sleep;

async fn hello(val: usize) {
    println!("==> {val}, {:?}", thread::current().id());
    log_mdc::insert("hello", val.to_string());
    sleep(Duration::new(2, 0)).await;

    let mut mdc = vec![];
    log_mdc::iter(|k, v| mdc.push((k.to_owned(), v.to_owned())));

    println!("<== {val}, {:?}, {:?}", thread::current().id(), mdc);
}

#[tokio::main(flavor = "multi_thread", worker_threads = 1)]
async fn main() {
    let now = Instant::now();
    let mut buffer = Vec::new();

    for i in 0..16 {
        // let handle = tokio::spawn(async move { hello(i).await });
        let handle = tokio::spawn(hello(i));
        buffer.push(handle);
    }

    futures::future::join_all(buffer).await;
    println!("<<< elapsed: {:.2?}", now.elapsed());
}
