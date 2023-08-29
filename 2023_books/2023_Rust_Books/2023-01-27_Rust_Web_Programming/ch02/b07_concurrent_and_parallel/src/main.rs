use async_std::task;
use chrono::{Local, SecondsFormat};
use futures::{executor::block_on, future::join_all, join};
use std::{thread, time};

fn main() {
    call01();
    call02();
    call03();
    call04();
    call05();
    call06();
}

fn local_now() -> String {
    Local::now().to_rfc3339_opts(SecondsFormat::Millis, true)
}

// 4s, main threads takes 2s, fut01 takes 2s, they are run in sequential
fn call01() {
    println!(">>> call01");
    let now = time::Instant::now();
    let fut01 = do01(1);

    let two_seconds = time::Duration::new(2, 0);
    println!("~~~ {} main thread sleep", local_now());
    thread::sleep(two_seconds);

    let outcome = block_on(fut01);
    println!("    elapsed {:?}, outcome: {}", now.elapsed(), outcome);
}

// 4s, fut01 and fut02 can't run in concurrent as thread::sleep(do01) is blocking
fn call02() {
    println!(">>> call02");
    let now = time::Instant::now();
    let fut03 = async {
        let fut01 = do01(2);
        let fut02 = do01(3);
        let results = join!(fut01, fut02);
        return results.0 + results.1;
    };

    let outcome = block_on(fut03);
    println!("    elapsed {:?}, outcome: {}", now.elapsed(), outcome);
}

// 2s, fut01 and fut03 run in concurrent
fn call03() {
    println!(">>> call03");
    let now = time::Instant::now();
    let fut03 = async {
        let fut01 = do02(2);
        let fut02 = do02(3);
        let results = join!(fut01, fut02);
        return results.0 + results.1;
    };

    let outcome = block_on(fut03);
    println!("    elapsed {:?}, outcome: {}", now.elapsed(), outcome);
}

// 2s, all routime(d01) in vec are run in parallel(threads)
fn call04() {
    println!(">>> call04");
    let now = time::Instant::now();

    let fut01 = async {
        let vec = vec![do01(3), do01(4), do01(5)];
        let handles = vec.into_iter().map(task::spawn).collect::<Vec<_>>();
        let results = join_all(handles).await;
        return results.into_iter().sum::<i8>();
    };

    let outcome = block_on(fut01);
    println!("    time elapsed {:?}, outcome: {}", now.elapsed(), outcome);
}

// 2s, all routime(d02) in vec are run in parallel(threads)
fn call05() {
    println!(">>> call05");
    let now = time::Instant::now();

    let fut01 = async {
        let vec = vec![do02(3), do02(4), do02(5)];
        let handles = vec.into_iter().map(task::spawn).collect::<Vec<_>>();
        let results = join_all(handles).await;
        return results.into_iter().sum::<i8>();
    };

    let outcome = block_on(fut01);
    println!("    elapsed {:?}, outcome: {}", now.elapsed(), outcome);
}

// 2s, all routime(d02) in vec are run in concurrent
fn call06() {
    println!(">>> call06");
    let now = time::Instant::now();

    let fut01 = async {
        let vec = vec![do02(3), do02(4), do02(5)];
        let results = join_all(vec).await;
        return results.into_iter().sum::<i8>();
    };

    let outcome = block_on(fut01);
    println!("    elapsed {:?}, outcome: {}", now.elapsed(), outcome);
}

async fn do01(number: i8) -> i8 {
    println!(
        "~~~ {} do01 number {} is running, thread id: {:?}",
        local_now(),
        number,
        thread::current().id()
    );
    let two_seconds = time::Duration::new(2, 0);
    thread::sleep(two_seconds);
    return 2 * number;
}

async fn do02(number: i8) -> i8 {
    println!(
        "~~~ {} do02 number {} is running, thread id: {:?}",
        local_now(),
        number,
        thread::current().id()
    );
    let two_seconds = time::Duration::new(2, 0);
    // thread::sleep(two_seconds);
    task::sleep(two_seconds).await;
    return 2 * number;
}
