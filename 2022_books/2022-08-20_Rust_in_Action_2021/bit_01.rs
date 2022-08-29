fn main() {
    //
    let a: u16 = 50115;
    let b: i16 = -15421;

    println!("{:016b}", a);
    println!("{:016b}", b);

    //
    let a: f32 = 42.24;
    let frankentype: u32 = unsafe { std::mem::transmute(a) };
    println!("{:032b}", frankentype);

    let b: f32 = unsafe { std::mem::transmute(frankentype) };
    println!("{}", b);
    assert_eq!(a, b);

    //
    let a: i32 = 1;
    println!("{:016b}", a);
}
