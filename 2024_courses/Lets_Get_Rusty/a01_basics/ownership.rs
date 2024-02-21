#![allow(dead_code)]

fn main() {
    let x: i32 = 42;
    let y: i32 = x; // for primitive types, a clone of value instead of take the onership
    println!("~~~ x: {x}, y: {y}");

    // s1, s2 is allocated in the stack, value string("Hello") is allocated in the heap
    // s1 owns the value, and then be move to s2
    let s1 = String::from("Rust");
    let mut s2 = s1; // s2 owns the value, the ownership is moved

    add_to_string(&mut s2);

    println!("~~~ s2: {s2}"); // s1 is invalid

    // References are pointers with rules/restrictions, and do not take ownership
    print1(&s2); // a borrow of s2
    print2(s2); // onwership of s2 is moved to name(parameter of print2)
}

// fn print_str2<S: AsRef<str>>(name: S)
fn print1(name: impl AsRef<str>) {
    let name = name.as_ref();
    println!("~~~ print1: {name}");
}

fn print2(name: String) {
    println!("~~~ print2: {name}");
}

fn add_to_string(s: &mut String) {
    s.push_str(" is Awesome!"); // automatically dereference (*s)
}

fn generate_str() -> String {
	String::from("Ferris")
}
