#![allow(unused_assignments, unused_variables)]

fn main() {
	// 1.
    let s1 = String::from("Hello, Rust!");
    let s2 = s1; // Move ownership from s1 to s2

    // println!("{}", s1); // This would cause a compile-time error because s1 is no longer valid.
    println!("{}", s2);

	// 2.
	let s1 = String::from("Hello, Rust!");
    let len = calculate_length(&s1); // Immutable borrow

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len() // Accessing the data through an immutable reference


	// 3.
    let r;
    {
        let s = String::from("Hello, Rust!");
        r = &s; // This would cause an error because s does not live long enough
    }
    // println!("{}", r); // r would be a dangling reference here
}
