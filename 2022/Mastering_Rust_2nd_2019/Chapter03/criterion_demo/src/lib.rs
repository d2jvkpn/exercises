// criterion_demo/src/lib.rs

pub fn slow_fibonacci(nth: usize) -> u64 {
    if nth <= 1 {
        return nth as u64;
    } else {
        return slow_fibonacci(nth - 1) + slow_fibonacci(nth - 2);
    }
}

pub fn fast_fibonacci(nth: usize) -> u64 {
    let (mut a, mut b, mut c) = (0, 1, 0);

    for _ in 1..nth {
        c = a + b;
        a = b;
        b = c;
    }
    c
}
