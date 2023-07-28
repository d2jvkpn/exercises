fn main() {
    let a: u8 = 0b_1010_1010;
    let b: u8 = 0b_0101_1010;

    // print
    println!("a = {a}, {a:08b}, b = {b}, {b:08b}");

    // logic gates
    println!("a AND b: {:08b}", a & b);
    println!("a OR b: {:08b}", a | b);
    println!("a XOR b: {:08b}", a ^ b);
    println!("NOT a: {:08b}", !a);

    // bitwise operations
    println!("a >> 1: {:08b}", a >> 1);
    println!("a << 1: {:08b}", a << 1);

    // big endian and little endian
    let n: u16 = 0x1234;
    println!("n is: {n:?}, {n:16b}, {n:0x}");

    let big_endian: [u8; 2] = n.to_be_bytes();
    let little_endian: [u8; 2] = n.to_le_bytes();
    println!(
        "n in big endia: {:02x}{:02X}\nn in little endian: {:02X}{:02X}",
        big_endian[0], big_endian[1], little_endian[0], little_endian[1]
    );
}
