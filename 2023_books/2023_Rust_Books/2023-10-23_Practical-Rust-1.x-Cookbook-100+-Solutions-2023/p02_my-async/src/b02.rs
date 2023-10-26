use std::time::Duration;
// use tokio::sync::mpsc;
use tokio::time::sleep;

async fn task1() {
    println!("Task 1 started");
    sleep(Duration::from_secs(2)).await;
    println!("Task 1 finished");
}

async fn task2() {
    println!("Task 2 started");
    sleep(Duration::from_secs(1)).await;
    println!("Task 2 finished");
}

async fn run_tasks() {
    let _ = tokio::join!(task1(), task2());
}

#[tokio::main]
async fn main() {
    run_tasks().await;
}
