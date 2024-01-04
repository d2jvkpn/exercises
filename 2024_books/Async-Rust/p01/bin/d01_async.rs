use chrono::Local;
use tokio::task::JoinHandle;

use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    thread,
    time::Duration,
};

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    // println!("Hello, wrold!");
    let counter_one = CounterFuture::new("one");
    let counter_two = CounterFuture::new("two").with_count(1);

    println!("~~~ {:?}", Local::now());
    thread::sleep(Duration::from_secs(1));

    // poll started
    let handle_one: JoinHandle<u32> = tokio::task::spawn(async { counter_one.await });

    // poll started
    let handle_two: JoinHandle<u32> = tokio::task::spawn(async { counter_two.await });

    let ans = tokio::join!(handle_one, handle_two);
    println!("==> {:?}", ans);
}

struct CounterFuture {
    name: String,
    count: u32,
}

impl CounterFuture {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string(), count: 0 }
    }

    pub fn with_count(mut self, count: u32) -> Self {
        self.count = count;
        self
    }
}

impl Future for CounterFuture {
    type Output = u32;

    fn poll(mut self: Pin<&mut Self>, ctx: &mut Context) -> Poll<Self::Output> {
        self.count += 1;

        println!("~~~ {:?} {} polling with result: {}", Local::now(), self.name, self.count);
        thread::sleep(Duration::from_secs(1));

        if self.count < 5 {
            ctx.waker().wake_by_ref();
            Poll::Pending
        } else {
            Poll::Ready(self.count)
        }
    }
}
