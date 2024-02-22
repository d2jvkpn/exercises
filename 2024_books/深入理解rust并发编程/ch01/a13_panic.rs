#![allow(unreachable_code)]

use std::{thread, time::Duration};

fn main() {
    panic_example();

    panic_caught_example();
}

fn panic_example() {
    let handle = thread::spawn(|| {
        thread::sleep(Duration::from_millis(1000));
        panic!("boom!");
        42
    });

    match handle.join() {
        Ok(v) => println!("==> All is well {v:?}"),
        Err(e) => eprintln!("!!! Got an error: {e:?}"),
    }
}

fn panic_caught_example() {
    let handle = thread::spawn(|| {
        thread::sleep(Duration::from_millis(1000));

        let result = std::panic::catch_unwind(|| {
            panic!("BOOM!");
        });

        println!("??? panic caught: {result:?}");
        42
    });

    match handle.join() {
        Err(e) => eprintln!("!!! Got and error: {e:?}"),
        Ok(v) => println!("==> All is well: {v}"),
    }
}
