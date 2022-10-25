pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums;
    let mut result: Vec<Vec<i32>> = Vec::new();
    if nums.len() < 3 {
        return result;
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

            result.push(vec![nums[i], nums[l], nums[r]]);

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

    result
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

pub fn group_anagrams(strs: Vec<&str>) -> Vec<Vec<&str>> {
    let mut strs = strs;
    // strs.sort_by(|a, b| a.len().partial_cmp(&b.len()).unwrap());
    // println!(">>> {:?}", strs);

    let (mut i, n, mut result) = (0, strs.len(), Vec::new());
    let mut bools = vec![false; n];

    while i < n {
        if bools[i] {
            i += 1;
            continue;
        }

        let mut chars = strs[i].chars().collect::<Vec<char>>();
        chars.sort();
        bools[i] = true;
        let mut j = i + 1;
        let mut list = vec![strs[i]];

        while j < n {
            if bools[j] {
                j += 1;
                continue;
            }

            let mut chars2 = strs[j].chars().collect::<Vec<char>>();
            chars2.sort();
            // println!("    {}, {:?}, {}, {:?}", i, chars, j, chars2);
            if chars == chars2 {
                list.push(strs[j]);
                bools[j] = true;
            }
            j += 1;
        }

        i += 1;
        result.push(list);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_three_sum() {
        let result = three_sum(vec![-1, 0, 1, 2, -1, -4]);
        println!("=== {:?}", result);
        // expected [[-1,-1,2],[-1,0,1]]

        let result = three_sum(vec![0, 0, 0, 0]);
        println!("=== {:?}", result);
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

    #[test]
    fn t_group_anagrams() {
        let strs = vec!["eat", "tea", "tan", "ate", "nat", "bat"];

        println!("{:?}", group_anagrams(strs));
    }
}
