#![allow(dead_code, unused_doc_comments, unused_variables, unused_unsafe)]

static mut COUNTER: u32 = 0;

fn main() {
    /// 1.unsafe block
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

    let mut s: String = "Let's Get Rusty".to_owned();

    /// 2. raw pointers
    let raw1 = &s as *const String;
    let raw2 = &mut s as *mut String;

    unsafe {
        (*raw2).push_str("!!!");
        println!("~~~ raw1: {}", *raw1);
    }
    // 1. to archive greater performance for the ability to interface with with other language, c, c++
    // 2. to build safe abstraction that the borrow checker doesn't understand
    // 3. to interface with hardware

    // invalid memory address
    let address = 0x012345usize;
    let raw3: *const String = address as *const String;
    unsafe {
        // Segmentation fault error
        // println!("~~~ raw3: {}", *raw3);
    }

    /// 3. unsafe function
    unsafe {
        my_func();
    }

    /// 4. unsafe trait
    println!("==> todo!()");
}

fn increment_counter() {
    unsafe {
        COUNTER += 1;
    }
}

unsafe fn my_func() {
    println!("==> calling my_func!");
}

unsafe trait MyTrait {
    fn call(&self);
}

unsafe impl MyTrait for String {
    fn call(&self) {
        println!("==> MyTrait::call")
    }
}
