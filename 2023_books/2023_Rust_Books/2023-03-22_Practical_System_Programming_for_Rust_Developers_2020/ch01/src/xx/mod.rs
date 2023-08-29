mod a1;
pub mod a2;

pub use a1::hello as a1_hello;

pub fn hello() {
    println!("Hello from xx!");
}
