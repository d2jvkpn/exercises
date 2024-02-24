// n!/k!/(n-k)!
pub fn combinations(nums: &[i32], k: usize) -> Vec<Vec<i32>> {
    fn rec(
        index: usize,
        current: &mut Vec<i32>,
        combs: &mut Vec<Vec<i32>>,
        nums: &[i32],
        k: usize,
    ) {
        if current.len() == k {
            combs.push(current.clone());
            return;
        } else if index > nums.len() {
            return;
        }

        for i in index..nums.len() {
            current.push(nums[i]);
            rec(i + 1, current, combs, nums, k);
            current.pop();
        }
    }

    let mut combs = vec![];
    let mut current = vec![];

    rec(0, &mut current, &mut combs, nums, k);

    combs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_combinations() {
        let ans = combinations(&[1, 2, 3, 4, 5], 2);
        assert_eq!(ans.len(), 10);

        let nums = (1..=10).collect::<Vec<_>>();
        let ans = combinations(&nums, 3);
        assert_eq!(ans.len(), 120);
    }
}
