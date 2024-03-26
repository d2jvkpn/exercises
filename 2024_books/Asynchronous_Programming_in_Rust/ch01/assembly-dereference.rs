use std::arch::asm;

fn main() {
    let mut t = 100;
    let t_ptr: *const usize = &t; // if you comment out this...
                                  // ...and uncomment the line below. The program will fail.
                                  // let t_ptr = 99999999999999 as *const usize;
    let x = dereference(t_ptr);

    t = 42;
    println!("t={}, x={}", t, x);
}

// #[cfg(target_arch = "x86-64")]
fn dereference(ptr: *const usize) -> usize {
    let mut res: usize;
    unsafe { asm!("mov {0}, [{1}]", out(reg) res, in(reg) ptr) };
    res
}

// FIX #11
#[cfg(target_arch = "aarch64")]
fn dereference(ptr: *const usize) -> usize {
    let mut res: usize;
    unsafe { asm!("ldr {0}, [{1}]", out(reg) res, in(reg) ptr) };
    res
}
