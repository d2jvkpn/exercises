// 200ms
#[path = "../misc.rs"]
mod misc;

use misc::now;
use std::{thread, time::Duration};
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    println!("start: {}", now());

    let f1 = async {
        let content = read_from_file1().await;
        println!("{:?}, thread: {:?}", content, thread::current().id());
    };
    let f2 = async {
        let content = read_from_file2().await;
        println!("{:?}, thread: {:?}", content, thread::current().id());
    };

    let _ = tokio::join!(f1, f2);
}

// function that simulates reading from a file, 100ms
async fn read_from_file1() -> String {
    println!("~~~ Processing file1: {}", now());
    sleep(Duration::from_millis(100)).await;
    format!("Hello, {}, there from file 1", now())
}

// function that simulates reading from a file, 200ms
async fn read_from_file2() -> String {
    println!("~~~ Processing file2: {}", now());
    sleep(Duration::from_millis(200)).await;
    format!("Hello, {}, there from file 2", now())
}
