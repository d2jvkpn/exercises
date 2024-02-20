use async_std::task;
use futures::{executor::block_on, future::FutureExt, pin_mut, select};

use std::time::Duration;

// Calling the following async fn returns a Future which does not
// implement Unpin
async fn call(sec: u64, val: usize) -> usize {
    task::sleep(Duration::new(sec, 0)).await;

    val
}

async fn run() {
    let fut_1 = call(1, 2).fuse();
    let fut_2 = call(1, 3).fuse();

    let mut fut_1 = Box::pin(fut_1); // Pins the Future on the heap
    pin_mut!(fut_2); // Pins the Future on the stack

    let res = select! {
        v = fut_1 => v,
        v = fut_2 => v,
    };

    println!("~~~ res: {res}");
    assert!(res == 1 || res == 2);
}

fn main() {
    block_on(run());
}
