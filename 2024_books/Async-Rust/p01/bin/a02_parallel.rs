use chrono::Local;

use std::{
    thread,
    time::{Duration, Instant},
};

fn main() {
    println!("==> sleep_seq");
    sleep_seq();

    println!("==> fib_parallel");
    fib_parallel(42);
}

fn sleep_seq() {
    fn task(id: isize) {
        let now = Local::now(); // now.format("%Y-%m-%dT%H:%M:%S%:Z"))
        println!("~~~ running task {}: {:?}...", id, now);
        thread::sleep(Duration::from_secs(1));
    }

    let start = Instant::now();

    for i in 0..10 {
        task(i);
    }

    let elapsed = start.elapsed();
    println!("--> sleep_seq took: {:?}", elapsed.as_millis());
}

fn fib_parallel(num: usize) {
    fn fibonacci(val: usize) -> usize {
        if val < 2 {
            return 1;
        }

        return fibonacci(val - 1) + fibonacci(val - 2);
    }

    let start = Instant::now();
    let mut handles = vec![];

    for _ in 0..4 {
        let handle = thread::spawn(move || -> (thread::ThreadId, usize) {
            let tid = thread::current().id();
            println!("--> task: {:?}", tid);
            (tid, fibonacci(num))
        });

        handles.push(handle);
    }

    for handle in handles {
        let ans = handle.join();
        println!("--> got ans: {ans:?}");
    }

    let duration: Duration = start.elapsed();
    println!("--> 4 threads fibonacci({}) took {:?}", num, duration);
}
