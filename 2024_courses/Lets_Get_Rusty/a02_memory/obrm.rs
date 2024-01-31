// Ownership Based Resouce Management
#![allow(dead_code)]
#![allow(unused_variables)]

use std::{fs, path, rc::Rc};

fn main() {
    // println!("Hello, wrold!");
}

struct Car {}

fn function_that_can_panic() {}

fn should_continue() -> bool {
    false
}

fn memory_exaple() {
    let car1 = Box::new(Car {});
    let car2 = car1; // variable car2 become the owner of heap allocated value, car1 was dropped
    let my_string = String::from("LGR");

    function_that_can_panic();

    if !should_continue() {
        return;
    }

    let car1 = Rc::new(Car {});
    let car2 = car1.clone(); // both car1 and car2 own the heap allocated value
}

fn file_example() {
    let p = path::Path::new("example.txt");
    let file = fs::File::open(&p).unwrap();

    function_that_can_panic();

    if !should_continue() {
        return;
    }
}
