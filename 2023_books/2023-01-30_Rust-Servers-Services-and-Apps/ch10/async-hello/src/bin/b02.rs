#[path = "../misc.rs"]
mod misc;

use misc::now;
use std::{
    thread::{self, sleep},
    time::Duration,
};

fn main() {
    println!("{} Hello before reading file", now());

    let handle1 = thread::spawn(|| {
        let contents = read_from_file1();
        println!("{:?}", contents);
    });

    let handle2 = thread::spawn(|| {
        let contents = read_from_file2();
        println!("{:?}", contents);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("{} Hello after reading file", now());
}

fn read_from_file1() -> String {
    sleep(Duration::new(4, 0));
    format!("Hello, {}, there from file 1", now())
}

fn read_from_file2() -> String {
    sleep(Duration::new(2, 0));
    format!("Hello, {}, there from file 2", now())
}
