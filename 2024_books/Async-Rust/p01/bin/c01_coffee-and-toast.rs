use tokio::time::{sleep, Duration};

use std::{thread, time::Instant};

// 3s+3s
async fn prep_coffee_mug() {
    println!("==> 1.1 Pouring milk...");
    thread::sleep(Duration::from_secs(3));
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

#[tokio::main]
async fn main() {
    // println!("Hello, wrold!");

    let start = Instant::now();

    let coffee_mug_step = prep_coffee_mug();
    let coffee_step = make_coffee();
    let toast_step = make_toast();

    // 6s(sync) + 3s(10s async) + 15s(10s async + 5s sync)
    tokio::join!(coffee_mug_step, coffee_step, toast_step); // 24s
    let elapsed = start.elapsed();

    println!("It took: {} seconds", elapsed.as_secs()); // 24s
}
