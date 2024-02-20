#![allow(dead_code, unused_assignments, unused_variables)]

const MAX_PLAYERS: u8 = 10;
static mut CASINO_NAME: &str = "Rusty Casino";

fn main() {
    println!("Hello, wrold!");

    // creation
    let a: i16 = 5;

    // mutability
    let mut b = 5;
    b = 10;

    // shadowing
    let c = 10;
    let c = 20;

    println!("~~~ c = {c}");

    // scope
    let d = 10;

    {
        let d = 40;
        println!("~~~ inner d = {d}");
        // inner d is dropped
    }
    // type aliasing
    type Age = u8;

    let a1: Age = 57;
    println!("~~~ outer d = {d}");

    // const
    println!("~~~ MAX_PLAYERS: {MAX_PLAYERS}");

    // funcition
    let ans = my_func(42);
    println!("~~~ ans: {ans}");

    // if/else
    let a: i32 = 5;
    if a > 5 {
        println!("~~~ bigger than 5");
    } else if a > 3 {
        println!("~~~ bigger than 3");
    } else {
        println!("~~~ smaller or equal to 3");
    }

    let b = if a > 5 { 1 } else { -1 };

    // loop
    'outer: loop {
        println!("~~~ loop forever");
        loop {
            break 'outer;
        }
    }

    let x = loop {
        break 5;
    };

    // while loop
    let mut a = 0;

    while a < 5 {
        println!("~~~ a is: {a}");
        a += 1;
    }

    // for loop
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    for e in arr {
        println!("~~~ element: {e}");
    }

    // comments

    // this is the name of my dog
    // I love him very much.
    /*
    this is a block comment.
    */
    let name = "Oreo";
}

fn data_types() {
    // boolean
    let b: bool = true;

    // unsigned integers
    let i1: u8 = 1;
    let i1: u16 = 1;
    let i1: u32 = 1;
    let i1: u64 = 1;
    let i1: u128 = 1;

    // signed integers
    let i1: i8 = 1;
    let i1: i16 = 1;
    let i1: i32 = 1;
    let i1: i64 = 1;
    let i1: i128 = 1;

    // floating point numbers
    let f1: f32 = 1.0;
    let f1: f64 = 1.0;

    // platform specific integers
    let p1: usize = 1;
    let p1: isize = 1;

    // characters, &str and String
    let c1: char = 'c';
    let s1: &str = "hello";
    let s1: String = String::from("hello");

    // arrays
    let a1: [i32; 5] = [1, 2, 3, 4, 5];
    let i1 = a1[4];

    // tuples
    let t1: (i32, i32, i32) = (1, 2, 3);
    let t1: (i32, f64, &str) = (5, 5.0, "5");

    let s1: &str = t1.2;
    let (i1, f1, s1) = t1;

    let unit: () = ();

    // type aliasing
    type Age = u8;

    let a1: Age = 57;
}

fn my_func(x: i32) -> i32 {
    println!("~~~ my_func called with: {x}");

    let y: i32 = 27;
    y
}
