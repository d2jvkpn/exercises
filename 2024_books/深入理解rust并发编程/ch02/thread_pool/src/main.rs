use rayon::ThreadPoolBuilder;
use threadpool::ThreadPool;

use std::{sync::mpsc, thread, time::Instant};

fn main() {
    let count = thread::available_parallelism().unwrap().get();

    let pool = ThreadPoolBuilder::new()
        .num_threads(count)
        .thread_name(|i| format!("worker-{:03}", i))
        .build()
        .unwrap();

    // .stack_size(), .start_handler()
    // .build_global().unwrap() // not recommended

    let num = 20; // 42

    let instant = Instant::now();
    let ans = fib1(num);
    println!("==> fib1(num): {ans}, elapsed: {:?}", instant.elapsed());

    let instant = Instant::now();
    let ans = pool.install(|| fib2(num));
    println!("==> fib2(num): {ans}, elapsed: {:?}", instant.elapsed());

    //
    let pool = ThreadPool::new(4);
    let (sender, receiver) = mpsc::channel();

    for i in 0..8 {
        let sender = sender.clone();

        pool.execute(move || {
            sender.send(i * 2).expect("!!! failed to send");
        });
    }
    drop(sender);

    while let Ok(ans) = receiver.recv() {
        println!("==> got: {}", ans);
    }
}

// Fibonacci
fn fib1(num: usize) -> usize {
    if num == 0 || num == 1 {
        return num;
    }

    fib1(num - 1) + fib1(num - 2)
}

fn fib2(num: usize) -> usize {
    if num == 0 || num == 1 {
        return num;
    }

    let (a, b) = rayon::join(|| fib2(num - 1), || fib2(num - 2));

    a + b
}
