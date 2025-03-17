fn main() {
    // println!("Hello, world!");

    // ch2.9
    let x = 255u8;
    let sum = x.wrapping_add(1);
    assert_eq!(sum, 0);

    let sum = x.saturating_add(1);
    assert_eq!(sum, 255);

    let sum = x.checked_add(1);
    assert!(sum.is_none());

    // ch2.10
    let a: u32 = 10;
    let b = a as u64;
    let c: u64 = a as _;

    // When converting to a u8, the Rust compiler will keep the last 8 bits of a u16 memory representation:
    let a: u16 = 255 + 1;
    let b = a as u8;
    assert_eq!(b, 0);
    println!("{b}");
}
