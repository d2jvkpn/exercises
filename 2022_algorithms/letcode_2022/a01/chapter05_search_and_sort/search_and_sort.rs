// nums1 = [1,2,3,0,0,0], m = 3, nums2 = [2,5,6], n = 3
// m = len(nums1) - len(nums2); n = len(nums2)
fn merge(nums1: &mut Vec<isize>, m: usize, nums2: &Vec<isize>, n: usize) {
    println!(">>> {:?}, {:?}", nums1, nums2);

    let (mut i, mut j) = (0, 0);

    while i < m + n && j < n {
        if nums1[i] > nums2[j] {
            (i + 1..nums1.len()).for_each(|idx| {
                nums1[idx] = nums1[idx - 1];
            });

            nums1[i] = nums2[j];
            j += 1;
        }
        i += 1;
    }

    if j < n {
        nums2[j..].iter().enumerate().for_each(|(idx, val)| {
            nums1[m + j + idx] = *val;
        });
    }

    println!("    {:?}", nums1);
}

fn main() {
    //
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let mut nums2 = vec![2, 5, 6];
    merge(&mut nums1, 3, &nums2, 3);

    nums1 = vec![-1, 0, 0, 3, 3, 3, 0, 0, 0];
    nums2 = vec![1, 2, 2];
    merge(&mut nums1, 6, &nums2, 3);

    nums1 = vec![0];
    nums2 = vec![1];
    merge(&mut nums1, 0, &nums2, 1);

    nums1 = vec![4, 0, 0, 0, 0, 0];
    nums2 = vec![1, 2, 3, 5, 6];
    merge(&mut nums1, 1, &nums2, 5);

    //
    println!("first_bad_version: {}", first_bad_version(10));
}

fn is_bad_version(version: i64) -> bool {
    version >= 4
}

fn first_bad_version(n: i64) -> i64 {
    let (mut lower, mut upper) = (1, n);
    let mut mid;

    loop {
        mid = (lower + upper) / 2;
        let is_bad = is_bad_version(mid);

        if is_bad && mid == 1 {
            return mid;
        } else if !is_bad && is_bad_version(mid + 1) {
            return mid + 1;
        }

        if is_bad {
            upper = mid
        } else {
            lower = mid
        }
    }
}
