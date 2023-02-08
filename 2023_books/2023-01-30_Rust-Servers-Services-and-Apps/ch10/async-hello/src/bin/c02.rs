// wrong concurrent with tokio, 6s

#[path = "../misc.rs"]
mod misc;

use misc::now;
use std::future::Future;
use std::{
    thread::{self, sleep},
    time::Duration,
};

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
    sleep(Duration::new(4, 0));
    format!("Hello, {}, there from file 1", now())
}

async fn read_from_file2() -> String {
    println!("{:?}, thread: {:?}", "Processing file 2", thread::current().id());
    sleep(Duration::new(2, 0));
    format!("Hello, {}, there from file 2", now())
}

#[allow(dead_code)]
fn read_from_file3() -> impl Future<Output = String> {
    async {
        println!("{:?}, thread: {:?}", "Processing file 3", thread::current().id());
        sleep(Duration::new(4, 0));
        format!("Hello, {}, there from file 1", now())
    }
}
