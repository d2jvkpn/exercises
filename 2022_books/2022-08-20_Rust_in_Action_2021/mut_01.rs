#![allow(dead_code)]

#[derive(Debug)]
struct Data {
    name: String,
    year: i64,
}

fn main() {
    let data = Data {
        name: "d2jvkpn".to_string(),
        year: 1997,
    };

    println!("data = {:?}", data);
    // data.year = 2017; // not mutable

    let mut d1 = data;
    // data not accessable
    d1.year = 2017;
    println!("d1 = {:?}", d1);
}
