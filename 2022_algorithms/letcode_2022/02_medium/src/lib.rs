pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums;
    let mut out: Vec<Vec<i32>> = Vec::new();
    if nums.len() < 3 {
        return out;
    }

    nums.sort();
    let (mut i, n) = (0, nums.len());

    while i < n - 1 {
        let (mut l, mut r) = (i + 1, n - 1);

        while l < r {
            let sum = nums[i] + nums[l] + nums[r];

            if sum > 0 {
                r -= 1;
                continue;
            } else if sum < 0 {
                l += 1;
                continue;
            }

            out.push(vec![nums[i], nums[l], nums[r]]);

            while l < r && nums[l] == nums[l + 1] {
                l += 1;
            }
            while l < r && nums[r] == nums[r - 1] {
                r -= 1;
            }
            l += 1;
            r -= 1;
        }

        while i < n - 1 && nums[i] == nums[i + 1] {
            i += 1;
        }

        i += 1;
    }

    out
}

pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    use std::collections::HashSet;
    let (mut rows, mut cols) = (HashSet::new(), HashSet::new());

    matrix.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, &v)| {
            if v == 0 {
                rows.insert(i);
                cols.insert(j);
            }
        });
    });

    matrix.iter_mut().enumerate().for_each(|(i, row)| {
        row.iter_mut().enumerate().for_each(|(j, v)| {
            if rows.contains(&i) || cols.contains(&j) {
                *v = 0;
            }
        });
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_three_sum() {
        let output = three_sum(vec![-1, 0, 1, 2, -1, -4]);
        println!("=== {:?}", output);
        // expected [[-1,-1,2],[-1,0,1]]

        let output = three_sum(vec![0, 0, 0, 0]);
        println!("=== {:?}", output);
        // expected [[-1,-1,2],[-1,0,1]]
    }

    #[test]
    fn t_set_zeroes() {
        let mut matrix = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        set_zeroes(&mut matrix);
        println!("{:?}", matrix);

        let mut matrix = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
        set_zeroes(&mut matrix);
        println!("{:?}", matrix);
    }
}
