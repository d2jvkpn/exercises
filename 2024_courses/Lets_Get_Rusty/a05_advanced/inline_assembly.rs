use std::arch::asm;

fn main() {
    println!("==> {}", add(1, 2));
}

fn add(x: u64, y: u64) -> u64 {
    let result: u64;

    unsafe {
        asm!("add {0}, {1}", inout(reg) x => result, in(reg) y);
    }

    result
}
