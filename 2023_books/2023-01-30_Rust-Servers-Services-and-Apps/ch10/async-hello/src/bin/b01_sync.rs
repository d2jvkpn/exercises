#[path = "../misc.rs"]
mod misc;

use misc::now;
use std::{thread::sleep, time::Duration};

fn main() {
    println!("{} Hello before reading file", now());
    let contents = read_from_file1();
    println!("{:?}", contents);
    println!("{} Hello after reading file", now());

    println!("{} Hello before reading file", now());
    let contents = read_from_file2();
    println!("{:?}", contents);
    println!("{} Hello after reading file", now());
}

fn read_from_file1() -> String {
    sleep(Duration::new(4, 0));
    String::from("Hello, there from file 1")
}

fn read_from_file2() -> String {
    sleep(Duration::new(2, 0));
    String::from("Hello, there from file 2")
}
