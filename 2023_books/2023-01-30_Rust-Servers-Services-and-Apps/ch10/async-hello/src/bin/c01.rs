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
    println!("{} Hello before reading file", now());

    let h01 = tokio::spawn(async {
        let contents = read_from_file1().await;
        println!("{:?}", contents);
    });

    let h02 = tokio::spawn(async {
        let contents = read_from_file2().await;
        println!("{:?}", contents);
    });

    let _ = tokio::join!(h01, h02); // start executing now

    println!("{} Hello after reading file", now());
}

async fn read_from_file1() -> String {
    println!(
        "{:?}, thread: {:?}",
        "Processing file 1",
        thread::current().id()
    );
    sleep(Duration::new(4, 0));
    format!("Hello, {}, there from file 1", now())
}

async fn read_from_file2() -> String {
    println!(
        "{:?}, thread: {:?}",
        "Processing file 2",
        thread::current().id()
    );
    sleep(Duration::new(2, 0));
    format!("Hello, {}, there from file 2", now())
}

#[allow(dead_code)]
fn read_from_file3() -> impl Future<Output = String> {
    async {
        println!(
            "{:?}, thread: {:?}",
            "Processing file 3",
            thread::current().id()
        );
        sleep(Duration::new(4, 0));
        format!("Hello, {}, there from file 1", now())
    }
}
