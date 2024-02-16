#![allow(dead_code)]

fn main() {
    let mut m = MyStruct {};

    let item = m.next();

    println!("~~~ next: {:?}", item);
}

trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

trait IntoIterator {
    type Item;
    type IntoIter: Iterator;

    fn into_iter(self) -> Self::IntoIter;
}

struct MyStruct {}

impl Iterator for MyStruct {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

/*
!!! conflicting implementation for `MyStruct`
impl Iterator for MyStruct {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
*/
