#![allow(unused_variables)]

use std::{fmt::Debug, ops::Deref};

fn main() {
    let a: i32 = 42;
    let b: Box<i32> = Box::new(a);

    let sptr1 = MySmartPointer::new(42);
    let sptr2 = MySmartPointer::new(50);
    assert_eq!(a, *sptr1);
    println!("{:?}", sptr1.deref());

    drop(sptr1); // without mannual drop, sptr2 will be drop firstly

    let vec = MySmartPointer::new(vec![1, 2, 3]);
    for v in &*vec {
        println!("~~~ This value is {}", v);
    }

    println!("==> {:?}", vec);
}

#[derive(Debug)]
struct MySmartPointer<T: Debug> {
    value: T,
}

impl<T: Debug> MySmartPointer<T> {
    fn new(value: T) -> Self {
        Self { value }
    }
}

impl<T: Debug> Deref for MySmartPointer<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.value
    }
}

impl<T: Debug> Drop for MySmartPointer<T> {
    fn drop(&mut self) {
        println!("dropping {:?}", self.value);
    }
}
