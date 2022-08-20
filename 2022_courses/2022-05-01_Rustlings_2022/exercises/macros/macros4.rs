// macros4.rs
// Make me compile! Execute `rustlings hint macros4` for hints :)

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };

    ($val:expr) => {
        println!(
            "{}::{} Look at this other macro: {}",
            file!(),
            line!(),
            $val
        );
        hello();
    };
}

fn main() {
    my_macro!();

    my_macro!(7777);
    my_macro!("Hello, world");
}

fn hello() {
    println!("Hello, world!");
}
