#![allow(dead_code)]

use std::borrow::Cow;

fn main() {
    call_1("x1");
}

fn call_1<S: AsRef<str>>(s: S) {
    let val = s.as_ref();
    println!("val: {val}");
}

fn abs_all(input: &mut Cow<[i32]>) {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
            input.to_mut()[i] = -v;
        }
    }
}
