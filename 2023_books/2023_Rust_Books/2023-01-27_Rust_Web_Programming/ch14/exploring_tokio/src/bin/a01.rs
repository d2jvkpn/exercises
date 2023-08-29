use std::{
    thread,
    time::{Duration, Instant},
};

async fn hello(input: i32) -> i32 {
    let id1 = thread::current().id();
    println!("--> hello: {input}, {id1:?}");
    tokio::time::sleep(Duration::from_secs(5)).await;

    let id2 = thread::current().id();
    println!("~~~ hello: {input}, {id2:?}");
    input
    // id1 may not equals to id2
}

// #[tokio::main(flavor = "multi_thread", worker_threads = 2)]
#[tokio::main]
async fn main() {
    let threads = thread::available_parallelism().unwrap().get();
    println!("All threads: {threads}");

    // DEMO 1
    println!(">>> DEMO 1");
    let now = Instant::now();

    let one = tokio::spawn(hello(1)); // start running now
    let two = tokio::spawn(hello(2));
    let three = tokio::spawn(hello(3));
    let four = tokio::spawn(hello(4));

    thread::sleep(Duration::from_millis(10));
    println!("==> Waiting...");
    // just like std::JoinHandle.join();
    let _ = one.await;
    let _ = two.await;
    let _ = three.await;
    let _ = four.await;

    println!("~~~ Elapsed: {:.3?}", now.elapsed());

    // DEMO 2
    println!(">>> DEMO 2");
    let now = Instant::now();

    let fut1 = hello(1); // not start running yet
    let fut2 = hello(2);
    let fut3 = hello(3);
    let fut4 = hello(4);

    thread::sleep(Duration::from_millis(10));
    println!("==> Joining...");
    // all futures run in the same thread(thread_id)
    let results = tokio::join!(fut1, fut2, fut3, fut4); // start running now
    println!("~~~ results: {results:?}");
    println!("~~~ Elapsed: {:.3?}", now.elapsed());
}
