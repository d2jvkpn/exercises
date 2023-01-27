#![allow(dead_code)]

use std::cell::{Cell, RefCell};

fn main() {
    println!("Hello, world!");

    let vec = RefCell::new(vec![0, 0, 0]);
    f2(&vec);
    println!("~~~ vec = {:?}", vec);
}

fn f1(a: &Cell<i32>, b: &Cell<i32>) {
    let before = a.get();
    b.set(b.get() + 1);
    let after = a.get();
    if before != after {
        println!("~~~ before != after");
    }
}

fn f2(v: &RefCell<Vec<i32>>) {
    v.borrow_mut().push(1);
}
