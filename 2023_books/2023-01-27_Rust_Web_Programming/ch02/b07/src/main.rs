use async_std::task;
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

// 4s, wrong
fn call01() {
    let now = time::Instant::now();
    let future_one = do01(1);

    let two_seconds = time::Duration::new(2, 0);
    thread::sleep(two_seconds);

    let outcome = block_on(future_one);
    println!(">>> call01 time elapsed {:?}", now.elapsed());
    println!("    Here is the outcome: {}", outcome);
}

// 4s, wrong
fn call02() {
    let now = time::Instant::now();
    let future_four = async {
        let outcome_one = do01(2);
        let outcome_two = do01(3);
        let results = join!(outcome_one, outcome_two);
        return results.0 + results.1;
    };

    let result = block_on(future_four);
    println!(">>> call02 time elapsed {:?}", now.elapsed());
    println!("    Here is the outcome: {}", result);
}

// 2s, correct
fn call03() {
    let now = time::Instant::now();
    let future_four = async {
        let outcome_one = do02(2);
        let outcome_two = do02(3);
        let results = join!(outcome_one, outcome_two);
        return results.0 + results.1;
    };

    let result = block_on(future_four);
    println!(">>> call03 time elapsed {:?}", now.elapsed());
    println!("    Here is the outcome: {}", result);
}

// 2s, wrong
fn call04() {
    let now = time::Instant::now();

    let async_outcome = async {
        let vec = vec![do01(3), do01(4), do01(5)];
        let handles = vec.into_iter().map(task::spawn).collect::<Vec<_>>();
        let results = join_all(handles).await;
        return results.into_iter().sum::<i8>();
    };

    let result = block_on(async_outcome);
    println!(">>> call04 time elapsed {:?}", now.elapsed());
    println!("    Here is the outcome: {}", result);
}

// 2s, correct
fn call05() {
    let now = time::Instant::now();

    let async_outcome = async {
        let vec = vec![do02(3), do02(4), do02(5)];
        let handles = vec.into_iter().map(task::spawn).collect::<Vec<_>>();
        let results = join_all(handles).await;
        return results.into_iter().sum::<i8>();
    };

    let result = block_on(async_outcome);
    println!(">>> call05 time elapsed {:?}", now.elapsed());
    println!("    Here is the outcome: {}", result);
}

// 2s, correct
fn call06() {
    let now = time::Instant::now();

    let async_outcome = async {
        let vec = vec![do02(3), do02(4), do02(5)];
        let results = join_all(vec).await;
        return results.into_iter().sum::<i8>();
    };

    let result = block_on(async_outcome);
    println!(">>> call06 time elapsed {:?}", now.elapsed());
    println!("    Here is the outcome: {}", result);
}

async fn do01(number: i8) -> i8 {
    println!("number {} is running", number);
    let two_seconds = time::Duration::new(2, 0);
    thread::sleep(two_seconds);
    return 2 * number;
}

async fn do02(number: i8) -> i8 {
    println!("number {} is running", number);
    let two_seconds = time::Duration::new(2, 0);
    // thread::sleep(two_seconds);
    task::sleep(two_seconds).await;
    return 2 * number;
}
