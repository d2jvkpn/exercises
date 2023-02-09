// 2s + 100ms

#[path = "../misc.rs"]
mod misc;

use misc::now;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    thread::{self, sleep},
    time::{Duration, Instant},
};

struct AsyncTimer {
    expiration_time: Instant,
}

impl Future for AsyncTimer {
    type Output = String;

    fn poll(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output> {
        let current_time = Instant::now();

        if current_time >= self.expiration_time {
            println!("Hello, it's time for Future 1, {}", now());
            return Poll::Ready(String::from("Future 1 has completed"));
        } else {
            println!("Hello, it's not yet time for Future 1, {}", now());
            let waker = ctx.waker().clone();
            sleep(self.expiration_time - current_time);
            waker.wake();
        }

        Poll::Pending
    }
}

#[tokio::main]
async fn main() {
    println!("start: {}", now());

    let f1 = async {
        let content =
            AsyncTimer { expiration_time: Instant::now() + Duration::from_millis(100) }.await;
        println!("{:?}, {}, thread: {:?}", content, now(), thread::current().id());
    };

    let f2 = async {
        let content = read_from_file2().await;
        println!("{:?}, thread: {:?}", content, thread::current().id());
    };

    let _ = tokio::join!(f1, f2);
}

// function that simulates reading from a file
fn read_from_file2() -> impl Future<Output = String> {
    async {
        println!("Processing file2: {}", now());
        sleep(Duration::new(2, 0));
        format!("Hello, {}, there from file 2", now())
    }
}
