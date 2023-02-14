// Rust Web Programing 2021, Chapter03
use async_std::task;
use chrono::{Local, SecondsFormat};
use futures::{executor::block_on, future::join_all, join};
use std::{
    any, thread,
    time::{Duration, Instant},
};

fn now_string() -> String {
    Local::now().to_rfc3339_opts(SecondsFormat::Millis, true)
}

fn run1(number: i8) -> i8 {
    //  DateTime<Local>
    // let now = || chrono::Local::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true);

    println!(
        " => {} Run1 number {:02} is running, {:?}",
        now_string(),
        number,
        thread::current().id()
    );
    let two_seconds = Duration::new(2, 0);
    thread::sleep(two_seconds);
    println!("    {} Run1 number {:02} is done", now_string(), number);
    return 2;
}

async fn run2(number: i8) -> i8 {
    println!(
        " => {} Run2 number {:02} is running, {:?}",
        now_string(),
        number,
        thread::current().id()
    );
    let two_seconds = Duration::new(2, 0);
    task::sleep(two_seconds).await;
    println!("    {} Run2 number {:02} is done", now_string(), number);
    return 2;
}

fn type_name<T>(_v: &T) -> String {
    format!("{}", any::type_name::<T>())
}

fn main() {
    // wrap a blocking function in async
    async fn run1_async(number: i8) -> i8 {
        run1(number)
    }

    let elapsed = |t: Instant| Duration::from_millis(t.elapsed().as_millis().try_into().unwrap());

    // #### 1 A future runs when block_on called
    println!(">>> {} Future_1 is created", now_string());
    let now = Instant::now();
    // defines a future
    let future_1 = run1_async(1);
    // holds the program for the result of the first future
    let outcome = block_on(future_1);
    println!("~~~ time elapsed {:?}, result: {}\n", elapsed(now), outcome);

    // #### 2 Futures(a blocking func wrapped in async func) can't run in concurrent
    println!(">>> {} Batch2 join!(...)", now_string());
    let now = Instant::now();
    // defines the async block for multiple futures (just like an async function)
    let batch2 = async {
        // defines two futures
        let future_2 = run1_async(2);
        let future_3 = run1_async(3);
        let future_4 = run1_async(4);
        // waits for both futures to complete in sequence
        return join!(future_2, future_3, future_4);
    };
    // holds the program for the result from the async block
    let results = block_on(batch2);
    //println!("~~~ time elapsed {:?}, results: {:?}\n", now.elapsed(), results);
    println!("~~~ time elapsed {:?}, results: {:?}\n", elapsed(now), results);

    // #### 3 Futures run in concurrent
    println!(">>> {} Batch3 join!(...)", now_string());
    let now = Instant::now();
    // defines the async block for multiple futures (just like an async function)
    println!("    created...");
    let batch3 = async {
        // defines two futures
        let future_5 = run2(5);

        // core::future::from_generator::GenFuture<async_functions::run2::{{closure}}>
        println!("    {}", type_name(&future_5));

        let future_6 = run2(6);
        let future_7 = run2(7);

        println!("    blocking sleep 1s");
        thread::sleep(Duration::new(1, 0));
        // waits for both futures to complete in sequence
        return join!(future_5, future_6, future_7); // !!! start running now
    };
    // holds the program for the result from the async block
    let results = block_on(batch3);
    println!("~~~ time elapsed {:?}, results: {:?}\n", elapsed(now), results);

    // #### 4 Futures run in concurrent and get results, task::spawn multiply tasks
    println!(">>> {} Batch4 join_all(...)", now_string());
    let now = Instant::now();
    println!("    created...");
    let batch4 = async {
        let futures_vec = vec![run1_async(8), run1_async(9), run1_async(10)];

        // applies the spawn async tasks for all futures and collect them into a vector
        let handles: Vec<task::JoinHandle<i8>> = futures_vec.into_iter().map(task::spawn).collect();
        // futures_vec.into_iter().map(task::spawn).collect::<Vec<_>>();

        let results = join_all(handles).await; // !!! in concurrent
        return results;
    };

    println!("    blocking sleep 1s");
    thread::sleep(Duration::new(1, 0));
    let results = block_on(batch4);
    println!("~~~ time elapsed {:?}, results: {:?}\n", elapsed(now), results);

    // #### 5 multithreading by using std
    println!(">>> {} Batch6 join threads", now_string());
    // start the timer again
    let now = Instant::now();
    // spawn a few functions with the same function
    println!("    created...");
    let batch5: Vec<thread::JoinHandle<i8>> =
        vec![11, 12, 13].into_iter().map(|v| thread::spawn(move || run1(v))).collect();
    // vec![thread::spawn(|| run(12)), thread::spawn(|| run(13)), thread::spawn(|| run(14))];

    println!("    blocking sleep 1s");
    thread::sleep(Duration::new(1, 0));
    let results: Vec<i8> = batch5.into_iter().map(|t| t.join().unwrap()).collect();
    // print the outcomes again from the threads
    println!("~~~ time elapsed {:?}, results: {:?}\n", elapsed(now), results);
}
