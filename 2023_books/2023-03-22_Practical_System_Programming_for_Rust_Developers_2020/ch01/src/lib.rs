//! Integration-test-example crate
//!
//! This is a library that contains functions related to dealing with processes
//! , and makes these tasks more convenient.

use std::process;
/// This function gets the process id of the current executable. It returns a non-zero number
/// ```
/// fn get_id() {
///     let x = ch01::get_process_id();
///     println!("{}",x);
/// }
/// ```
pub fn get_process_id() -> u32 {
    process::id()
}

pub mod call_a;
mod call_b;
mod xx;

#[cfg(test)]
mod tests {
    #[test]
    fn hello_lib() {
        println!("Hello from hello_lib");
        assert_eq!(2 + 2, 4);
    }
}
