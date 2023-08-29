pub fn hello() {
    println!("Hello from lib call_a!");
    crate::call_b::hello();

    // call_b isn't public in lib.rs
    // ch01::call_b::hello();
}
