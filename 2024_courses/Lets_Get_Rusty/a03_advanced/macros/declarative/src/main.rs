#![allow(dead_code, unused_variables)]

use declarative::*;

use std::collections::HashMap;

fn main() {
    // println!("Hello, world!");

    hello!(); // hello!{}; hello![];

    // let scores: HashMap<String, i32> = HashMap::new();
    // let mut scores2: HashMap<String, i32> = HashMap::new();
    let mut scores1 = map!(String, i32);

    scores1.insert("Red team".into(), 3);
    scores1.insert("Blue team".to_string(), 5);
    scores1.insert("Green team".to_owned(), 2);

    let scores2 = map!(
        "Red team".into() => 3,
        "Blue team".to_string() => 5,
        "Green team".to_owned() => 2
    );

    for (team, score) in &scores2 {
        println!("--> team: {team}, score: {score}");
    }
}
