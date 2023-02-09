// 2s + 100ms
#[path = "../misc.rs"]
mod misc;

use misc::now;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    thread,
    time::{Duration, Instant},
};
use tokio::time::sleep;

struct AsyncTimer {
    expiration_time: Instant,
}

impl Future for AsyncTimer {
    type Output = String;

    fn poll(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output> {
        let current_time = Instant::now();

        if current_time > self.expiration_time {
            println!("~~~ Future 1 isn't ready, {}", now());
            return Poll::Ready(String::from("Future 1 has completed"));
        } else {
            println!("~~~ Future 1 isn't ready, {}", now());
            let waker = ctx.waker().clone();

            let delta: Duration = self.expiration_time - current_time;
            if delta > Duration::new(0, 0) {
                // TODO: thread::sleep is blocking, not a real async, how to interact with kernel and impls this
                thread::sleep(delta);
            }
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
        println!("~~~ Processing file2: {}", now());
        sleep(Duration::new(2, 0)).await;
        format!("Hello, {}, there from file 2", now())
    }
}
