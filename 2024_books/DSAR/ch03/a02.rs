fn main() {
    let s1 = String::from("Hello, Rust!");
    let len = calculate_length(&s1); // Immutable borrow

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len() // Accessing the data through an immutable reference
}
