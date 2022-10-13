pub fn max_profit_v1(prices: &Vec<i32>) -> i32 {
    // let max_val = prices.iter().max();

    // let max =
    //    prices.iter().enumerate().fold(
    //        (0, i32::MIN),
    //        |t, v| if t.1 > *v.1 { t } else { (v.0, *v.1) },
    //    );

    // let min_val = prices[0..max.0].iter().min().unwrap_or(&0);

    let mut profit = 0;

    prices[0..prices.len() - 1]
        .iter()
        .enumerate()
        .for_each(|(idx, val)| {
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
        println!("~~~ {}, {}, {}", price, no_hold, hold);
        no_hold = max(no_hold, hold + *price);
        hold = max(hold, -*price);
        println!("    no_hold={}, hold={}", no_hold, hold);
    });

    return no_hold;
}

fn main() {
    let input = vec![7, 1, 5, 3, 6, 4];
    println!("{}", max_profit(&input));
}
