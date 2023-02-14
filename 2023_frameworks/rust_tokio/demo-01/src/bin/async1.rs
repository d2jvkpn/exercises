use std::{
    thread,
    time::{Duration, Instant},
};

async fn hello(input: i32) -> i32 {
    let id1 = thread::current().id();
    println!("--> hello: {input}, {id1:?}");
    tokio::time::sleep(Duration::from_secs(2)).await;

    let id2 = thread::current().id();
    println!("<-- hello: {input}, {id2:?}");
    input
    // id1 may not equals to id2
}

async fn hello_blocking(input: i32) -> i32 {
    let id1 = thread::current().id();
    println!("--> hello_blocking: {input}, {id1:?}");
    std::thread::sleep(Duration::from_secs(2));

    let id2 = thread::current().id();
    println!("~~~ hello_blocking: {input}, {id2:?}");
    input
    // id1 may not equals to id2
}

// #[tokio::main(flavor = "multi_thread", worker_threads = 2)]
#[tokio::main]
async fn main() {
    let threads = thread::available_parallelism().unwrap().get();
    println!("All threads: {threads}");

    // DEMO 1
    println!("==> DEMO 1, tokio::spawn async funcs, ASYNC");
    let now = Instant::now();

    let one = tokio::spawn(hello(1)); // running now
    let two = tokio::spawn(hello(2)); // running now
    let three = tokio::spawn(hello(3)); // running nowd

    thread::sleep(Duration::from_millis(10));
    println!("=== Waiting...");
    // just like std::JoinHandle.join();
    // let _ = one.await; // waitting for exit
    // let _ = two.await; // waitting for exit
    // let _ = three.await; // waitting for exit
    let _ = tokio::join!(one, two, three);

    println!("~~~ Elapsed: {:.3?}", now.elapsed());

    // DEMO 2
    println!("==> DEMO 2, tokio::join multipy futures, ASYNC");
    let now = Instant::now();

    let fut4 = hello(4); // not running yet
    let fut5 = hello(5); // not running yet
    let fut6 = hello(6); // not running yet

    thread::sleep(Duration::from_millis(10));
    println!("=== Joining...");
    // all futures run in the same thread(thread_id)
    let results = tokio::join!(fut4, fut5, fut6); // start running now
    println!("~~~ results: {results:?}");
    println!("~~~ Elapsed: {:.3?}", now.elapsed());

    // DEMO 3
    println!("==> DEMO 3 tokio::spawn blocking funcs, ASYNC");
    let now = Instant::now();

    let seven = tokio::spawn(hello(7)); // running now
    let eight = tokio::spawn(hello(8)); // running now
    let nine = tokio::spawn(hello(9)); // running now

    thread::sleep(Duration::from_millis(10));
    println!("=== Waiting...");
    let _ = tokio::join!(seven, eight, nine);

    println!("~~~ Elapsed: {:.3?}", now.elapsed());

    // DEMO 4
    println!("==> DEMO 4, tokio::join multipy blocking futures, SYNC");
    let now = Instant::now();

    let fut10 = hello_blocking(10); // not running yet
    let fut11 = hello_blocking(11); // not running yet
    let fut12 = hello_blocking(12); // not running yet

    thread::sleep(Duration::from_millis(10));
    println!("=== Joining...");
    let results = tokio::join!(fut10, fut11, fut12); // start running now
    println!("~~~ results: {results:?}");
    println!("~~~ Elapsed: {:.3?}", now.elapsed());
}
