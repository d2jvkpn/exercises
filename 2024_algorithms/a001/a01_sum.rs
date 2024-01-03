use std::time::SystemTime;

fn main() {
    // println!("Hello, wrold!");
    let mut start;

    println!("==> sum_seq1(500000) used");
    for _ in 0..10 {
        start = SystemTime::now();
        let _ = sum_seq1(500000);
        println!("  {} ms", start.elapsed().unwrap().as_millis());
    }

    println!("==> sum_seq2(500000) used");
    for _ in 0..10 {
        start = SystemTime::now();
        let _ = sum_seq2(500000);
        println!("  {} ns", start.elapsed().unwrap().as_nanos());
    }
}

fn sum_seq1(n: i64) -> i64 {
    let mut sum = 0;

    for i in 1..=n {
        sum += i;
    }

    sum
}

fn sum_seq2(n: i64) -> i64 {
    n * (n + 1) / 2
}
