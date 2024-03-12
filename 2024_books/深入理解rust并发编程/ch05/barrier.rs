use std::{
    sync::{Arc, Barrier},
    thread,
};

fn main() {
    let num = 3;
    let barrier = Arc::new(Barrier::new(num));
    let mut handles = Vec::with_capacity(num);

    for id in 0..num {
        let barrier = Arc::clone(&barrier);

        let handle = thread::spawn(move || {
            println!("==> {:?} working: {}", thread::current().id(), id);
            thread::sleep(std::time::Duration::from_secs(id as u64));

            barrier.wait();

            println!("=== {:?} resumed: {}", thread::current().id(), id);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
