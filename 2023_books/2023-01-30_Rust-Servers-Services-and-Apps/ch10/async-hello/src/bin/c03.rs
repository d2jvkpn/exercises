// correct concurrent with tokio, 4s

#[path = "../misc.rs"]
mod misc;

use misc::now;
// use std::future::Future;
use std::{thread, time::Duration};
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    println!("{} Hello before reading file, thread: {:?}", now(), thread::current().id());

    let f01 = read_from_file1();

    let f02 = read_from_file2();

    let results = tokio::join!(f01, f02); // start executing now
    println!("{:?}", results);

    println!("{} Hello after reading file", now());
}

async fn read_from_file1() -> String {
    println!("{:?}, thread: {:?}", "Processing file 1", thread::current().id());
    sleep(Duration::new(4, 0)).await;
    format!("Hello, {}, there from file 1", now())
}

async fn read_from_file2() -> String {
    println!("{:?}, thread: {:?}", "Processing file 2", thread::current().id());
    sleep(Duration::new(2, 0)).await;
    format!("Hello, {}, there from file 2", now())
}
