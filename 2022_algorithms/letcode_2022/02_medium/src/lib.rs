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
    // strs.sort_by(|a, b| a.len().partial_cmp(&b.len()).unwrap());
    // println!(">>> {:?}", strs);

    let (mut i, n) = (0, strs.len());
    let mut result = Vec::new();
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

pub fn group_anagrams2(strs: Vec<&str>) -> Vec<Vec<&str>> {
    // strs.sort_by(|a, b| a.len().partial_cmp(&b.len()).unwrap());
    // println!(">>> {:?}", strs);

    use std::collections::HashMap;

    let (mut i, n) = (0, strs.len());
    let mut result = Vec::new();
    let mut bools = vec![false; n];

    let (mut map0, mut map1) = (HashMap::new(), HashMap::new());

    while i < n {
        if bools[i] {
            i += 1;
            continue;
        }

        map0.clear();
        strs[i].chars().for_each(|v| {
            let count = map0.entry(v).or_insert(0);
            *count += 1;
        });
        bools[i] = true;
        let mut j = i + 1;
        let mut list = vec![strs[i]];

        while j < n {
            if bools[j] {
                j += 1;
                continue;
            }

            map1.clear();
            strs[j].chars().for_each(|v| {
                let count = map1.entry(v).or_insert(0);
                *count += 1;
            });
            // println!("    {}, {:?}, {}, {:?}", i, chars, j, chars2);
            if map0 == map1 {
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

pub fn length_of_longest_substring(s: String) -> i32 {
    use std::collections::HashSet;

    let mut result = 0;
    let chars = s.chars().collect::<Vec<char>>();
    let len = chars.len();
    // let mut map = HashSet::from([chars[i]]);
    let mut set = HashSet::new();

    for i in 0..len {
        let (mut left, mut right) = (i, i);
        set.clear();
        set.insert(chars[i]);

        while right < len - 1 && !set.contains(&chars[right + 1]) {
            set.insert(chars[right + 1]);
            right += 1;
        }

        while left > 0 && !set.contains(&chars[left - 1]) {
            set.insert(chars[left - 1]);
            left -= 1;
        }

        let l = right - left + 1;
        if l > result {
            result = l;
        }
    }

    result as i32
}

pub fn longest_palindrome(s: String) -> String {
    let mut result: &[char] = &[];
    let chars = s.chars().collect::<Vec<char>>();
    let len = chars.len();

    for i in 0..len {
        let (mut left, mut right) = (i, i);
        while left > 0 && chars[left - 1] == chars[i] {
            left -= 1;
        }

        while right < len - 1 && chars[right + 1] == chars[i] {
            right += 1;
        }

        while left > 0 && right < len - 1 && chars[left - 1] == chars[right + 1] {
            left -= 1;
            right += 1;
        }

        // println!("~~~ {}, {}, {}", left, i, right);
        if right - left + 1 > result.len() {
            result = &chars[left..right + 1];
        }
    }

    result.iter().collect()
}

pub fn increasing_triplet(nums: Vec<i32>) -> bool {
    let (mut small, mut mid) = (i32::MAX, i32::MAX);

    for i in 0..nums.len() {
        let v = nums[i];
        if v <= small {
            small = v;
        } else if v <= mid {
            mid = v;
        } else {
            return true;
        }
    }

    false
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

        let strs = vec!["eat", "tea", "tan", "ate", "nat", "bat"];
        println!("{:?}", group_anagrams2(strs));
    }

    #[test]
    fn t_length_of_longest_substring() {
        assert_eq!(3, length_of_longest_substring("pwwkew".to_string()));
    }

    #[test]
    fn t_longest_palindrome() {
        let result = longest_palindrome("babad".to_string());
        println!("{:?}", result);
        assert_eq!(&result[..], "bab");
    }

    #[test]
    fn t_increasing_triplet() {
        assert_eq!(increasing_triplet(vec![2, 1, 5, 0, 4, 6]), true);
        assert_eq!(increasing_triplet(vec![5, 4, 3, 2, 1]), false);
        assert_eq!(increasing_triplet(vec![1, 2, 3, 4, 5]), true);
    }
}
