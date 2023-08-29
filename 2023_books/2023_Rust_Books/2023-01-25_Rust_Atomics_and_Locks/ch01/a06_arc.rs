use std::{sync::Arc, thread};

fn main() {
    let a = Arc::new([1, 2, 3]);

    let a1 = a.clone();
    let t1 = thread::spawn(move || dbg!(a1));

    let a2 = a.clone();
    let t2 = thread::spawn(move || dbg!(a2));

    t1.join().unwrap();
    t2.join().unwrap();
}
