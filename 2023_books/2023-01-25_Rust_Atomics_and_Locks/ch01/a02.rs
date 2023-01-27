use std::{iter::FromIterator, thread, io};

fn main() {
    let mut val = String::new();
    println!("Enter a number(unsize):");
    io::stdin().read_line(&mut val).unwrap();
    let val: usize = val.trim().parse().unwrap();

    let numbers = Vec::from_iter(0..val);
    dbg!(&numbers);

    let t = thread::spawn(move || {
        let len = numbers.len();

        let sum = numbers.into_iter().sum::<usize>();
        sum / len // len may be zero
    });

    match t.join() {
        Ok(v) => println!("average: {v}"),
        Err(e) => eprintln!("!!! failed to calculate avarage: {:?}", e),
    }
}

// release
// $ rustc -C opt-level=3 -C debuginfo=0 a02.rs
