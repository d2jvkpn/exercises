fn main() {
    let mut m = MyStruct {};

    let item: Option<String> = m.next();
    println!("~~~ item: {:?}", item);

    let item: Option<i32> = m.next();
    println!("~~~ item: {:?}", item);
}

/*
trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
*/

// generic iterator
trait Iterator<Item> {
    fn next(&mut self) -> Option<Item>;
}

struct MyStruct {}

impl Iterator<String> for MyStruct {
    fn next(&mut self) -> Option<String> {
        None
    }
}

impl Iterator<i32> for MyStruct {
    fn next(&mut self) -> Option<i32> {
        None
    }
}
