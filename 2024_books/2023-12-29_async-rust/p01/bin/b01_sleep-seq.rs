use std::{
    thread::sleep,
    time::{Duration, Instant},
};

use chrono::Local;

fn main() {
    // println!("Hello, wrold!");

    let start = Instant::now();

    for i in 0..10 {
        task(i);
    }

    let elapsed = start.elapsed();
    println!("==> The whole program took: {:?}", elapsed.as_millis());
}

fn task(id: isize) {
    let now = Local::now(); // now.format("%Y-%m-%dT%H:%M:%S%:Z"))
    println!("~~~ running task {}: {:?}...", id, now);
    sleep(Duration::from_secs(1));
}
