#![allow(dead_code)]

use std::fmt::Debug;

fn main() {
    println!("Hello, world!");

    let val1 = MyStruct1;
    println!("{}", val1.do_something());

    let val2 = MyStruct2 { value: 42 };
    do_somthing2(val2);

    let my_enum = MyEnum1::Value(42);
    println!("{:?}", my_enum);

    match_my_enum(&my_enum);
    
	let val1 = MyEnum2::Value1;
	assert_eq!(val1 as i32, 1);
}

// fn do_somthing2(item: impl MyTrait2<i32>) {
fn do_somthing2<T: MyTrait2<i32>>(item: T) {
    println!("ans: {:?}", item.do_something("Hello".to_string()));
}

fn match_my_enum(value: &MyEnum1) {
    match value {
        MyEnum1::X1 => println!("~~~ X1"),
        MyEnum1::Value(v) => println!("~~~ ans value: {}", v),
        MyEnum1::Name(ref v) => println!("~~~ ans name: {}", v),
        MyEnum1::Point { x, y: _, .. } => println!("ans point x: {}", x),
        _ => {},
    }
}

//
trait MyTrait1 {
    type Output;

    fn do_something(&self) -> Self::Output;
}

struct MyStruct1;

impl MyTrait1 for MyStruct1 {
    type Output = i32;

    fn do_something(&self) -> Self::Output {
        42
    }
}

//
trait MyTrait2<T> {
    fn do_something<U>(&self, value: U) -> (T, U);
}

struct MyStruct2<T> {
    value: T,
}

impl<T: Copy + Debug> MyTrait2<T> for MyStruct2<T> {
    fn do_something<U>(&self, value: U) -> (T, U) {
        (self.value, value)
    }
}

//
#[derive(Debug, PartialEq, Eq)]
#[repr(usize)]
enum MyEnum1 {
    X1 = 0,
    X2 = 1,
    Value(i32),
    Name(String),
    Point { x: i32, y: i32, z: i32 },
}

enum MyEnum2 {
	Value1 = 1,
	Value2 = 2,
	Value3 = 3,
}
