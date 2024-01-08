fn main() {
    // println!("Hello, wrold!");
    let cashes = [1, 5, 10, 20, 50];
    println!("~~~ cashes: {:?}", cashes);

    let amount = 31_u32;
    let ans = min_cashes_rec(&cashes, amount);
    println!("==> need refund cashes for {}: {}", amount, ans);
    assert_eq!(ans, 3);
    assert_eq!(min_cashes_rec(&cashes, 41), 3);

    for t in &[(31, 3), (41, 3), (51, 2), (60, 2), (82, 5)] {
        let ans = min_cashes_dp(&cashes, t.0);

        println!("==> need refund cashes for {}: {}", t.0, t.1);
        assert_eq!(ans, t.1);
    }
}

// too many calculations for numbers like 51, 81
pub fn min_cashes_rec(cashes: &[u32], amount: u32) -> u32 {
    let mut ans = amount;
    let mut temp;

    // dbg!(&amount);

    if amount == 0 {
        return 0;
    }

    if cashes.contains(&amount) {
        return 1;
    }

    for c in cashes.iter().filter(|&v| *v <= amount).collect::<Vec<&u32>>() {
        temp = 1 + min_cashes_rec(&cashes, amount - c);
        if temp < ans {
            ans = temp;
        }
    }

    ans
}

pub fn min_cashes_dp(cashes: &[u32], amount: u32) -> u32 {
    let (mut value, mut temp): (u32, u32);
    let mut index: usize;
    let mut min_cashes: Vec<u32> = vec![0; (amount + 1) as usize];

    for denm in 1..=amount {
        value = denm;

        for c in cashes.iter().filter(|&v| *v <= denm).collect::<Vec<&u32>>() {
            //  dbg!("~~~ denm={}, c={}", denm, *c);
            index = (denm - *c) as usize;

            temp = 1 + min_cashes[index];
            if temp < value {
                value = temp;
            }
        }

        min_cashes[denm as usize] = value;
    }

    min_cashes[amount as usize]
}
