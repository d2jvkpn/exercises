use std::{
    sync::{Arc, Condvar, Mutex},
    thread,
    time::{Duration, Instant},
};

fn main() {
    let mutex = Arc::new(Mutex::new(false));
    let condvar = Arc::new(Condvar::new());
    let mut handles = Vec::with_capacity(3);

    for id in 0..3 {
        let mutex = Arc::clone(&mutex);
        let condvar = Arc::clone(&condvar);

        let handle = thread::spawn(move || {
            let mut guard = mutex.lock().unwrap();

            while !*guard {
                println!("~~~ {:?} {}", Instant::now(), id);
                guard = condvar.wait(guard).unwrap();
            }

            println!("Thread {} woke up", id);
        });

        handles.push(handle);
    }

    thread::sleep(Duration::from_secs(7));

    {
        let mut guard = mutex.lock().unwrap();
        *guard = true;
        condvar.notify_all();
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
