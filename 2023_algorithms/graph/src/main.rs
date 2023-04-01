#![allow(unused_variables)]
#![allow(dead_code)]

use std::collections::HashMap;

// a simple struct for graph
#[derive(Debug)]
struct Node {
    val: String,
    to: Vec<String>,
}

impl Node {
    fn new(val: &str, to: Vec<&str>) -> Self {
        Self {
            val: val.into(),
            to: to.iter().map(|v| v.to_string()).collect(),
        }
    }
}

fn main() {
    let mut map = HashMap::new();
    map.insert("A", Node::new("A", vec!["B", "C"]));
    map.insert("B", Node::new("B", vec!["D", "E"]));
    map.insert("C", Node::new("C", vec!["F"]));
    map.insert("D", Node::new("D", vec!["F"]));
    map.insert("E", Node::new("E", vec!["F"]));
    map.insert("F", Node::new("F", vec!["G"]));
    map.insert("G", Node::new("G", vec![]));

    let start = "A";
    let end = "G";

    println!("{:?}", map);
}
