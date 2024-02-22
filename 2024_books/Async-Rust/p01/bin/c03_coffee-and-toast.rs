use tokio::time::{sleep, Duration};

use std::{thread, time::Instant};

// 100ms+3s
async fn prep_coffee_mug() {
    println!("==> 1.1 Pouring milk...");
    sleep(Duration::from_millis(100)).await;
    println!("~~~ 1.1 milk poured.");

    println!("==> 1.2 Putting instant coffee...");
    thread::sleep(Duration::from_secs(3));
    println!("~~~ 1.2 instant coffee put.");
}

// 10s+3s
async fn make_coffee() {
    println!("==> 2.1 Boiling kettle...");
    sleep(Duration::from_secs(10)).await;
    println!("~~~ 2.1 kettle boiled.");

    println!("==> 2.2 Bouring boiled water...");
    thread::sleep(Duration::from_secs(3));
    println!("~~~ 2.2 boiled water poured.");
}

// 10s+5s
async fn make_toast() {
    println!("==> 3.1 Putting bread in toaster...");
    sleep(Duration::from_secs(10)).await;
    println!("~~~ 3.1 bread toasted.");

    println!("==> 3.2 Buttering toasted bread...");
    thread::sleep(Duration::from_secs(5));
    println!("~~~ 3.2 toasted bread buttered.");
}

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    // println!("Hello, wrold!");

    let start = Instant::now();

    let person_one = tokio::task::spawn(async {
        prep_coffee_mug().await;
        make_coffee().await;
        make_toast().await;
    });

    let _ = person_one.await;

    let elapsed = start.elapsed();

    println!("It took: {} seconds", elapsed.as_secs()); // 31s
}
