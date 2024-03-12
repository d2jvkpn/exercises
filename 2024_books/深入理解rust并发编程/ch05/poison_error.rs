use std::{
    sync::{Arc, Mutex},
    thread,
};
// std::sync::{PoisonError, LockResult}

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..5 {
        let counter = Arc::clone(&counter);

        let handle = thread::spawn(move || match counter.lock() {
            Ok(mut num) => {
                *num += 1;
                if *num == 3 {
                    panic!("Simulated panic!");
                }
            }
            Err(e) => {
                println!("Thread encountered a poisoned lock: {:?}", e);
                let mut num = e.into_inner();
                *num += 1;
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        if let Err(e) = handle.join() {
            println!("!!! {e:?}");
        }
    }

    println!("Final count: {:?}", counter.lock().unwrap());
}
