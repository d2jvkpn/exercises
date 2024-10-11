fn main() {
    let s1 = String::from("Hello, Rust!");
    let s2 = s1; // Move ownership from s1 to s2

    // println!("{}", s1); // This would cause a compile-time error because s1 is no longer valid.
    println!("{}", s2);
}
