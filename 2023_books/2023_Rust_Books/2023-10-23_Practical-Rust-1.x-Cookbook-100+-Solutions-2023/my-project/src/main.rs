use std::fmt::Debug;

fn main() {
    println!("Hello, world!");

    let val1 = MyStruct1;
    println!("{}", val1.do_something());

    let val2 = MyStruct2 { value: 42 };
    do_somthing2(val2);

    let my_enum = MyEnum::Value(42);
    println!("{:?}", my_enum);

    match_my_enum(my_enum);
}

// fn do_somthing2(item: impl MyTrait2<i32>) {
fn do_somthing2<T: MyTrait2<i32>>(item: T) {
    println!("ans: {:?}", item.do_something("Hello".to_string()));
}

fn match_my_enum(value: MyEnum) {
    match value {
        MyEnum::XX => println!("~~~ XX"),
        MyEnum::Value(v) => println!("~~~ ans value: {}", v),
        MyEnum::Name(ref v) => println!("~~~ ans name: {}", v),
        MyEnum::Point { x, y: _, .. } => println!("ans point x: {}", x),
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
#[derive(Debug)]
enum MyEnum {
    XX,
    Value(i32),
    Name(String),
    Point { x: i32, y: i32, z: i32 },
}
