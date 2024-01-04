use std::{
    sync::Arc,
    thread,
    time::Duration,
};

use tokio::{
    sync::Mutex,
    task::{spawn, JoinHandle},
};

#[tokio::main]
async fn main() {
    let shared_data = Arc::new(Mutex::new(SharedData { counter: 0 }));

	let data = shared_data.clone();
    let handle_one: JoinHandle<()> =
        spawn(async { count(3, data, CounterType::Increment).await });

	let data = shared_data.clone();
    let handle_two: JoinHandle<()> =
        spawn(async { count(3, data, CounterType::Decrement).await });

    let (_, _) = tokio::join!(handle_one, handle_two);
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

async fn count(count: u32, data: Arc<Mutex<SharedData>>, counter_type: CounterType) {
    for _ in 0..count {
        let mut data = data.lock().await;

        match counter_type {
            CounterType::Increment => {
                data.increment();
                println!("after increment: {}", data.counter);
            }

            CounterType::Decrement => {
                data.decrement();
                println!("after decrement: {}", data.counter);
            }
        };

        thread::sleep(Duration::from_secs(1));
    }
}
