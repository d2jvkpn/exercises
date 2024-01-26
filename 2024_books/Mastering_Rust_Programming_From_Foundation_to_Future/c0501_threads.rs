use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    // println!("Hello, wrold!");

    //
    let data = vec![1, 2, 3, 4];

    let handle = thread::spawn(move || {
        println!("~~~ Hello from the spawned thread!");
        println!("~~~ Data: {:?}", data);
    });

    handle.join().unwrap();

    println!("~~~ Back in the main thread.");

    //
    arc_counting();
    arc_vec();
}

fn arc_counting() {
    let counter = Arc::new(Mutex::new(0));
    let num = 10;
    let mut handles = Vec::with_capacity(num);

    for _ in 0..num {
        let counter = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            let mut value = counter.lock().unwrap();
            println!("~~~ thread id: {:?}, value={}", thread::current().id(), value);
            *value += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("==> Final count: {:?}", *counter.lock().unwrap());
}

fn arc_vec() {
    let data = Arc::new(vec![1, 2, 3, 4, 5]);
    let num = 10;
    let mut handles = Vec::with_capacity(num);

    for _ in 0..num {
        let d = Arc::clone(&data);

        let handle = thread::spawn(move || {
            let sum: i32 = d.iter().sum();
            println!("~~~ thread id: {:?}, sum: {}", thread::current().id(), sum);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
