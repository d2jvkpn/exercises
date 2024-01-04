use std::{
    thread,
    time::{Duration, Instant},
};

fn main() {
    // println!("Hello, wrold!");
    let start = Instant::now();
    let mut handles = vec![];

    for _ in 0..4 {
        let handle = thread::spawn(|| -> usize {
            println!("==> task: {:?}", thread::current().id());
            fibonacci(42)
        });

        handles.push(handle);
    }

    for handle in handles {
        let ans = handle.join();
        println!("==> got ans: {ans:?}");
    }

    let duration: Duration = start.elapsed();
    println!("==> 4 threads fibonacci(42) took {:?}", duration);
}

fn fibonacci(val: usize) -> usize {
    if val < 2 {
        return 1;
    }

    return fibonacci(val - 1) + fibonacci(val - 2);
}
