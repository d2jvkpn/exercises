#![feature(thread_spawn_unchecked)]
use std::{thread, time::Duration};

fn main() {
    start_one_thread();
    start_n_threads();
    start_thread_by_builder();
    thread_spawn_unchecked();
    start_thread_with_yield_now();
}

fn start_one_thread() {
    let handle: thread::JoinHandle<_> = thread::spawn(|| {
        println!("Hello from a thread1: {:?}", thread::current().id());
    });

    // handle.join().unwrap();
    if let Err(e) = handle.join() {
        eprintln!("error: {e:?}");
    };
}

fn start_n_threads() {
    const N: usize = 10;

    let handles: Vec<thread::JoinHandle<usize>> = (0..N)
        .map(|v| {
            thread::spawn(move || {
                println!("--> Hello from thread{}!", v);
                v
            })
        })
        .collect();

    for handle in handles {
        println!("--> {:?}", handle.join());
    }
}

fn start_thread_by_builder() {
    let builder = thread::Builder::new().name("foo".into()).stack_size(32 * 1024);

    let handle = builder
        .spawn(|| {
            let tid = thread::current();

            println!("==> START, id={:?}, name={:?}", tid.id(), tid.name());
            thread::park();
            // thread::park_timeout(Duration::from_secs(5));
            println!("==> EXIT");
        })
        .unwrap();

    println!("~~~ sleep 2");
    thread::sleep(Duration::from_secs(2));
    println!("~~~ unpark");
    handle.thread().unpark();

    handle.join().unwrap();
}

fn thread_spawn_unchecked() {
    let builder = thread::Builder::new();

    let x = 1;
    let thread_x = &x;

    let handle = unsafe {
        builder
            .spawn_unchecked(move || {
                println!("x={}", *thread_x);
            })
            .unwrap()
    };

    handle.join().unwrap();
}

pub fn start_thread_with_yield_now() {
    let handle1 = thread::spawn(|| {
        thread::yield_now();
        println!("yield_now!");
    });

    let handle2 = thread::spawn(|| {
        thread::yield_now();
        println!("yield_now in another thread!");
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}
