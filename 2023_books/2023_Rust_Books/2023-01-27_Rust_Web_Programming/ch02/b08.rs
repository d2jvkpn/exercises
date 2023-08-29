use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    //
    let names = Arc::new(vec!["dave", "chloe", "simon"]);
    let reference_data = Arc::clone(&names);

    let t1 = thread::spawn(move || {
        println!("{}", reference_data[1]);
    });

    //
    let count = Mutex::new(0);

    let t2 = thread::spawn(move || {
        let mut val = count.lock().unwrap();
        *val += 1;
        println!("count={}", val);
    });

    t1.join().unwrap();
    t2.join().unwrap();
}
