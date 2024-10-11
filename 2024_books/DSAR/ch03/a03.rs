#![allow(unused_assignments, unused_variables)]

fn main() {
    let r;
    {
        let s = String::from("Hello, Rust!");
        r = &s; // This would cause an error because s does not live long enough
    }
    // println!("{}", r); // r would be a dangling reference here
}
