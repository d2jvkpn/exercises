#![allow(unused_variables)]

fn main() {
    //
    let v1: Vec<String> = Vec::new();
    let mut v1: Vec<String> = Vec::with_capacity(5);

    v1.push(String::from("one"));
    v1.push(String::from("two"));
    v1.push(String::from("three"));

    let s: &String = &v1[0];
    println!("~~~ v1: {:?}, s: {:?}", v1, s);

    let s: String = v1.remove(1);
    println!("~~~ v1: {:?}, s: {:?}", v1, s);

    println!("~~~ v1.get(1): {:?}", v1.get(1));

    for s in &mut v1 {
        s.push_str("!!!");
    }
    println!("~~~ v1: {v1:?}");

    //
    let v2 = vec![1, 2, 3];
}
