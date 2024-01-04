use tokio::time::{self, timeout, Duration};

async fn slow_task(secs: u64) -> &'static str {
    time::sleep(Duration::from_secs(secs)).await;

    println!("~~~ slow task completed");
    "Slow Task Completed"
}

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    let duration = Duration::from_secs(5);
    let result = timeout(duration, slow_task(6)).await;

    match result {
        Ok(value) => println!("==> Task completed success: {}", value),
        Err(_) => eprintln!("==> Task timed out!"),
    }

    time::sleep(Duration::from_secs(10)).await; // if timeout happened, slow_task will be dropped
    eprintln!("<== Exit");
}
