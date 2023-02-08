#[path = "../misc.rs"]
mod misc;

use misc::now;
use std::{thread::sleep, time::Duration};

fn main() {
    println!("{} Hello before reading file", now());
    let contents = read_from_file();
    println!("{:?}", contents);
    println!("{} Hello after reading file", now());
}

fn read_from_file() -> String {
    sleep(Duration::new(2, 0));
    String::from("Hello, there")
}
