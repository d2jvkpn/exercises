fn main() {
    // println!("Hello, wrold!");

    for t in &[(0, 0), (1, 1), (2, 1), (10, 55), (19, 4181)] {
        assert_eq!(fibonacci_rec(t.0), t.1);
        assert_eq!(fibonacci_dp(t.0), t.1);
    }
}

fn fibonacci_rec(n: usize) -> usize {
    match n {
        0 => 0,
        1 | 2 => 1,
        _ => fibonacci_rec(n - 1) + fibonacci_rec(n - 2),
    }
}

fn fibonacci_dp(n: usize) -> usize {
    match n {
        0 => return 0,
        1 | 2 => return 1,
        _ => {}
    };

    let mut dp = [1, 1];

    for i in 2..=n {
        dp[i % 2] += dp[(i - 1) % 2];
    }

    dp[(n - 1) % 2]
}
