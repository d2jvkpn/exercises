use std::time::SystemTime;

fn main() {
    // println!("Hello, wrold!");
    let mut start;

    for _ in 0..10 {
        start = SystemTime::now();
        let _ = sum_of_n1(500000);
        println!("sum_of_n1 used {} ns", start.elapsed().unwrap().as_millis());
    }

    for _ in 0..10 {
        start = SystemTime::now();
        let _ = sum_of_n2(500000);
        println!("sum_of_n2 used {} ns", start.elapsed().unwrap().as_nanos());
    }
}

fn sum_of_n1(n: i64) -> i64 {
    let mut sum = 0;

    for i in 1..=n {
        sum += i;
    }

    sum
}

fn sum_of_n2(n: i64) -> i64 {
    n * (n + 1) / 2
}
