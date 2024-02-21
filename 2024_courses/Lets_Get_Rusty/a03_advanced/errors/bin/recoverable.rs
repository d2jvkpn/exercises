#![allow(unused_variables)]

use std::fs;

fn main() {
    match get_user_id("") {
        Err(e) => eprintln!("!!! got error: {e:?}"),
        Ok(v) => println!("==> got user_id: {v}"),
    }

    match fs::File::open("data/not_exists.txt") {
        Ok(v) => println!("~~~ file: {v:?}"),
        Err(e) => eprintln!("!!! open file error: {e:?}"),
    };

    let file = fs::File::open("data/not_exist.txt").unwrap();
    let file = fs::File::open("data/not_exist.txt").expect("!!! failed to open file");
}

fn get_user_id(username: &str) -> Result<i32, &'static str> {
    if username.is_empty() {
        Err("username can't be empty")
    } else {
        Ok(42)
    }
}
