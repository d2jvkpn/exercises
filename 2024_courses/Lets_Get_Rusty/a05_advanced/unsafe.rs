static mut COUNTER: u32 = 0;

fn main() {
    // println!("Hello, wrold!");

    for _ in 0..10 {
        increment_counter();
    }

    unsafe {
        // 1. Derefernce a  raw pointer
        // 2. Call an unsafe function
        // 3. Implement an unsafe trait
        // 4. Access/Modify a mutable static variable
        // 5. Access fields of a union
        println!("~~~ COUNTER: {}", COUNTER);
    }
}

fn increment_counter() {
    unsafe {
        COUNTER += 1;
    }
}
