// Time: O(n*2^n), Space: O(n), nums is sorted
pub fn subsets_without_dups<T: Clone + PartialEq>(nums: &[T]) -> Vec<Vec<T>> {
    // TODO: overflow
    let mut subsets = Vec::with_capacity(2_usize.pow(nums.len() as u32)); // Vec::new();
    let mut curset = Vec::with_capacity(nums.len());

    helper(0, nums, &mut curset, &mut subsets);

    subsets
}

fn helper<T>(mut i: usize, nums: &[T], curset: &mut Vec<T>, subsets: &mut Vec<Vec<T>>)
where
    T: Clone + PartialEq,
{
    if i >= nums.len() {
        subsets.push(curset.clone());
        return;
    }

    while i + 1 < nums.len() && nums[i] == nums[i + 1] {
        i += 1;
    }

    curset.push(nums[i].clone());
    helper(i + 1, nums, curset, subsets);

    curset.pop();
    helper(i + 1, nums, curset, subsets);
}

// cargo test --lib -- t_subsets_without_dups --nocapture
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_subsets_without_dups() {
        let nums = vec![1, 2, 3, 4];

        let subsets = subsets_without_dups(&nums);
        dbg!(&subsets);
        assert_eq!(subsets.len(), 16);

        let nums = vec![1, 2, 2, 3, 4];
        let subsets = subsets_without_dups(&nums);
        dbg!(&subsets);
        assert_eq!(subsets.len(), 16);
    }
}
