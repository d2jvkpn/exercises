#![allow(unused_variables)]

use std::ops::Deref;
use List::{Cons, Nil};

fn main() {
    //
    let v1 = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    println!("{:?}", v1);

    //
    let a: i32 = 42;
    let b: Box<i32> = Box::new(a);

    let sptr1 = MySmartPointer::new(42);
    let sptr2 = MySmartPointer::new(50);
    assert_eq!(a, *sptr1);
    println!("{}", sptr1.deref());

    drop(sptr1); // without mannual drop, sptr2 will be drop firstly
}

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MySmartPointer {
    value: i32,
}

impl MySmartPointer {
    fn new(value: i32) -> Self {
        Self { value }
    }
}

impl Deref for MySmartPointer {
    type Target = i32;

    fn deref(&self) -> &i32 {
        &self.value
    }
}

impl Drop for MySmartPointer {
    fn drop(&mut self) {
        println!("dropping {:?}", self.value);
    }
}
