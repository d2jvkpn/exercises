use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    let num = 15;
    let vec = Arc::new(Mutex::new(Vec::with_capacity(num)));
    let mut handles = Vec::with_capacity(num);

    for i in 0..num {
        let vec = vec.clone();

        let handle = thread::spawn(move || {
            let mut vec = vec.lock().unwrap();
            vec.push(i);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("==> final vec: {:?}", vec.lock().unwrap());
}
