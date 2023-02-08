#[path = "../misc.rs"]
mod misc;

use misc::now;

fn main() {
    println!("{} Hello before reading file", now());
    let contents = read_from_file();
    println!("{:?}", contents);
    println!("{} Hello after reading file", now());
}

fn read_from_file() -> String {
    String::from("Hello, there")
}
