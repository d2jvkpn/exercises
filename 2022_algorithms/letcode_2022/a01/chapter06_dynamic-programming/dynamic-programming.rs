#![allow(dead_code)]

pub fn max_profit_v1(prices: &Vec<i32>) -> i32 {
    // let max_val = prices.iter().max();

    // let max =
    //    prices.iter().enumerate().fold(
    //        (0, i32::MIN),
    //        |t, v| if t.1 > *v.1 { t } else { (v.0, *v.1) },
    //    );

    // let min_val = prices[0..max.0].iter().min().unwrap_or(&0);

    let mut profit = 0;

    prices[0..prices.len() - 1].iter().enumerate().for_each(|(idx, val)| {
        let p = *prices[idx + 1..].iter().max().unwrap_or(&0) - val;
        profit = if p > profit { p } else { profit };
    });

    profit
}

// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iv/discuss/1363872/rust-general-dp-of-stock-problems
pub fn max_profit_v2(prices: &Vec<i32>) -> i32 {
    use std::cmp::max;

    let k: usize = 1;
    let mut dp: Vec<Vec<i32>> = vec![vec![0, std::i32::MIN]; k + 1];
    println!(">>> -, {:?}", dp);

    //    let n: usize = prices.len();
    //    for i in 1..=n {
    //        for k in 1..=k {
    //            dp[k][0] = max(dp[k][0], prices[i - 1] + dp[k][1]);
    //            dp[k][1] = max(dp[k][1], dp[k - 1][0] - prices[i - 1]);
    //        }
    //    }

    prices.iter().for_each(|price| {
        for i in 1..=k {
            dp[i][0] = max(dp[i][0], dp[i][1] + *price);
            dp[i][1] = max(dp[i][1], dp[i - 1][0] - *price);

            println!("    {}, {:?}", price, dp);
        }
    });

    dp[k][0]
}

pub fn max_profit(prices: &Vec<i32>) -> i32 {
    use std::cmp::max;

    if prices.len() < 2 {
        return 0;
    }

    let (mut hold, mut no_hold) = (-prices[0], 0);

    prices[1..].iter().for_each(|price| {
        println!("~~~ price={}, no_hold={}, hold={}", price, no_hold, hold);
        // keep not hold or sellout
        no_hold = max(no_hold, hold + *price);

        // buy
        hold = max(hold, -*price);

        println!("    no_hold={}, hold={}", no_hold, hold);
    });

    return no_hold;
}

fn max_sub_array(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let max_val = |x1: i32, x2: i32| if x1 > x2 { x1 } else { x2 };

    let (mut cur, mut max) = (nums[0], nums[0]);

    nums[1..].iter().for_each(|x| {
        cur = max_val(cur, 0) + *x;
        max = max_val(max, cur);
    });

    return max;
}

pub fn rob(nums: Vec<i32>) -> i32 {
    // todo!()
    if nums.is_empty() {
        return 0;
    }

    let max_val = |x1: i32, x2: i32| if x1 > x2 { x1 } else { x2 };
    let (mut dp0, mut dp1) = (0, nums[0]);

    nums[1..].iter().for_each(|val| {
        let temp = max_val(dp0, dp1);
        dp1 = dp0 + *val;
        dp0 = temp;
    });

    max_val(dp0, dp1)
}

fn climb_stairs_v1(n: i32) -> i32 {
    if n <= 1 {
        return 1;
    }
    if n < 3 {
        return n;
    }
    println!("~~~ n={}", n);
    climb_stairs_v1(n - 1) + climb_stairs_v1(n - 2)
}

fn climb_stairs_v2(n: i32) -> i32 {
    // fibonacci, a: last climb is 1, b: last climb is 2
    fn fib(n: i32, a: i32, b: i32) -> i32 {
        if n <= 1 {
            return b;
        }
        println!("~~~ n={}, a={}, b={}", n, a, b);
        fib(n - 1, b, a + b)
    }

    fib(n, 1, 1)
}

fn climb_stairs(n: i32) -> i32 {
    if n <= 1 {
        return 1;
    }

    let len = n as usize;
    let mut dp = vec![0; len + 1];
    dp[1] = 1;
    dp[2] = 2;

    for i in 3..=len {
        dp[i] = dp[i - 1] + dp[i - 2];
    }

    dp[len]
}

fn main() {
    let input = vec![7, 1, 5, 3, 6, 4];
    println!(">>> max_profit: {}", max_profit(&input));

    println!(">>> max_sub_array: {:?}", max_sub_array(vec![5, 4, -1, 7, 8]));

    println!(">>> rob: {:?}", rob(vec![2, 7, 9, 3, 1]));

    // println!(">>> climb_stairs_v1: {:?}", climb_stairs_v1(45));
    println!(">>> climb_stairs: {:?}", climb_stairs(45));
}
