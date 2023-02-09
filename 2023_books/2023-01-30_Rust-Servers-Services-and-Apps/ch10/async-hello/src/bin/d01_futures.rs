#[path = "../misc.rs"]
mod misc;

use misc::now;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    thread::{self, sleep},
    time::Duration,
};

struct ReadFileFuture {}

impl Future for ReadFileFuture {
    type Output = String;

    fn poll(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("{:?}, {}", "Tokio! stop polling me", now());
        // Poll::Pending

        ctx.waker().wake_by_ref();
        Poll::Ready(format!("Hello, {}, there from file 1", now()))
    }
}

#[tokio::main]
async fn main() {
    println!("{} Hello before reading file", now());

    let f1 = async {
        let content = ReadFileFuture {}.await;
        println!("{:?}, thread: {:?}", content, thread::current().id());
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
        sleep(Duration::new(2, 0));
        format!("Hello, {}, there from file 2", now())
    }
}
