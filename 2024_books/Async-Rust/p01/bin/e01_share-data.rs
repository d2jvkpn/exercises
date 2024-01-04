use std::{
    future::Future,
    mem,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll},
    thread,
    time::Duration,
};

use tokio::task::{spawn, JoinHandle};

#[tokio::main]
async fn main() {
    let shared_data = Arc::new(Mutex::new(SharedData { counter: 0 }));

    let counter_one =
        CounterFuture { counter_type: CounterType::Increment, data: shared_data.clone(), count: 0 };

    let counter_two =
        CounterFuture { counter_type: CounterType::Decrement, data: shared_data.clone(), count: 0 };

    let handle_one: JoinHandle<u32> = spawn(async { counter_one.await });
    let handle_two: JoinHandle<u32> = spawn(async { counter_two.await });

    let ans = tokio::join!(handle_one, handle_two);
    println!("{:?}", ans);
}

#[derive(Debug)]
enum CounterType {
    Increment,
    Decrement,
}

#[derive(Debug)]
struct SharedData {
    counter: i32,
}

impl SharedData {
    fn increment(&mut self) {
        self.counter += 1;
    }

    fn decrement(&mut self) {
        self.counter -= 1;
    }
}

#[derive(Debug)]
struct CounterFuture {
    counter_type: CounterType,
    data: Arc<Mutex<SharedData>>,
    count: u32,
}

impl Future for CounterFuture {
    type Output = u32;

    fn poll(mut self: Pin<&mut Self>, ctx: &mut Context) -> Poll<Self::Output> {
        thread::sleep(Duration::from_secs(1));

        let mut guard = match self.data.try_lock() {
            Ok(v) => v,
            Err(e) => {
                eprintln!("error: {}", e);
                ctx.waker().wake_by_ref();
                return Poll::Pending;
            }
        };

        match self.counter_type {
            CounterType::Increment => {
                guard.increment();
                println!("after increment: {}", guard.counter);
            }

            CounterType::Decrement => {
                guard.decrement();
                println!("after decrement: {}", guard.counter);
            }
        };

        mem::drop(guard);
        self.count += 1;

        if self.count < 3 {
            ctx.waker().wake_by_ref();
            Poll::Pending
        } else {
            Poll::Ready(self.count)
        }
    }
}
