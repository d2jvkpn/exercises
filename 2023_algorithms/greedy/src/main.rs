// It follows local optimum choice at each stage with intend to find the global optimum

#![allow(dead_code)]

fn main() {
    println!("Hello, world!");
}

fn min_coins(coins: Vec<i32>, target: i32) -> Option<Vec<i32>> {
    let mut ans = Vec::new();
    let mut added;

    loop {
        let sum: i32 = ans.iter().sum();
        if sum == target {
            return Some(ans);
        }

        added = false;
        for v in &coins {
            if sum + v > target {
                continue;
            }

            ans.push(*v);
            added = true;
            break;
        }

        if !added {
            return None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_min_coins() {
        assert_eq!(min_coins(vec![20, 10, 5], 45), Some(vec![20, 20, 5]));
        assert_eq!(min_coins(vec![20, 10, 5], 30), Some(vec![20, 10]));
    }
}
