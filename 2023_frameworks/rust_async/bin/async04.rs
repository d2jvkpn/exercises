// Rust Web Programing 2021, Chapter03
use async_std::task;
use chrono::{Local, SecondsFormat};
use futures::{executor::block_on, future::join_all, join};

use std::{
    thread,
    time::{Duration, Instant},
};

fn main() {
    // wrap a blocking function in async
    async fn async_blocking(number: i8) -> i8 {
        hello_blocking(number)
    }
    // let elapsed = |t: Instant| Duration::from_millis(t.elapsed().as_millis().try_into().unwrap());

    // #### 1. A future runs when block_on called
    println!("==> {} Future1 is created", now_string());
    let now = Instant::now();
    // defines a future
    let fut1 = async_blocking(1);
    // holds the program for the result of the first future
    let outcome = block_on(fut1);
    println!("--> result: {}, elapsed: {:?}\n", outcome, now.elapsed());

    // #### 2. Futures(a blocking func wrapped in async func) can't run in concurrent
    println!("==> {} Batch2 join!(...)", now_string());
    let now = Instant::now();
    // defines the async block for multiple futures (just like an async function)
    let batch2 = async {
        // defines two futures
        let fut2 = async_blocking(2);
        let fut3 = async_blocking(3);
        let fut4 = async_blocking(4);
        // waits for both futures to complete in sequence
        return join!(fut2, fut3, fut4);
    };
    // holds the program for the result from the async block
    let results = block_on(batch2);
    println!("--> results: {:?}, elapsed: {:?}\n", results, now.elapsed());

    // #### 3. Futures run in concurrent
    println!("==> {} Batch3 join!(...)", now_string());
    let now = Instant::now();
    // defines the async block for multiple futures (just like an async function)
    println!("--> created...");
    let batch3 = async {
        // defines two futures
        let fut5 = hello_async(5);
        // core::future::from_generator::GenFuture<async_functions::hello_async::{{closure}}>

        let fut6 = hello_async(6);
        let fut7 = hello_async(7);

        println!("--> blocking sleep 1s");
        thread::sleep(Duration::new(1, 0));
        // waits for both futures to complete in sequence
        return join!(fut5, fut6, fut7); // !!! start running now
    };
    // holds the program for the result from the async block
    let results = block_on(batch3);
    println!("--> results: {:?}, elapsed: {:?}\n", results, now.elapsed());

    // #### 4. Futures run in concurrent and get results, task::spawn multiply tasks
    println!("==> {} Batch4 join_all(...)", now_string());
    let now = Instant::now();
    println!("--> created...");
    let batch4 = async {
        let futures_vec = vec![async_blocking(8), async_blocking(9), async_blocking(10)];

        // applies the spawn async tasks for all futures and collect them into a vector
        let handles: Vec<task::JoinHandle<i8>> = futures_vec.into_iter().map(task::spawn).collect();
        // futures_vec.into_iter().map(task::spawn).collect::<Vec<_>>();

        let results = join_all(handles).await; // !!! in concurrent
        return results;
    };

    println!("--> blocking sleep 1s");
    thread::sleep(Duration::new(1, 0));
    let results = block_on(batch4);
    println!("--> results: {:?}, elapsed: {:?}\n", results, now.elapsed());

    // #### 5. multithreading by using std
    println!("==> {} Batch6 join threads", now_string());
    // start the timer again
    let now = Instant::now();
    // spawn a few functions with the same function
    println!("--> creating...");
    let batch5: Vec<thread::JoinHandle<i8>> =
        vec![11, 12, 13].into_iter().map(|v| thread::spawn(move || hello_blocking(v))).collect();
    // vec![thread::spawn(|| hello_blocking(12)), thread::spawn(|| hello_blocking(13)), thread::spawn(|| hello_blocking(14))];

    println!("--> blocking sleep 1s");
    thread::sleep(Duration::new(1, 0));
    let results: Vec<i8> = batch5.into_iter().map(|t| t.join().unwrap()).collect();
    // print the outcomes again from the threads
    println!("--> results: {:?}, elapsed: {:?}\n", results, now.elapsed());
}

fn hello_blocking(number: i8) -> i8 {
    //  DateTime<Local>
    // let now = || chrono::Local::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true);

    println!(
        "==> {} hello_blocking number {:02} is running, {:?}",
        now_string(),
        number,
        thread::current().id()
    );

    let two_seconds = Duration::new(2, 0);
    thread::sleep(two_seconds);
    println!("--~ {} hello_blocking number {:02} is done", now_string(), number);

    return 2;
}

async fn hello_async(number: i8) -> i8 {
    println!(
        "==> {} hello_async number {:02} is running, {:?}",
        now_string(),
        number,
        thread::current().id()
    );

    let two_seconds = Duration::new(2, 0);
    task::sleep(two_seconds).await;
    println!("--~ {} hello_async number {:02} is done", now_string(), number);

    return 2;
}

fn now_string() -> String {
    Local::now().to_rfc3339_opts(SecondsFormat::Millis, true)
}
