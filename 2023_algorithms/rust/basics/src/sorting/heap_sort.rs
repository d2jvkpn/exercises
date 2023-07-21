use std::fmt::Debug;

pub fn min_heap_sort<T: Ord + Debug>(slice: &mut [T]) {
    if slice.len() <= 1 {
        return;
    }

    fn help<T: Ord + Debug>(slice: &mut [T]) {
        for i in (0..slice.len() / 2).rev() {
            heapify(slice, i, |a, b| a < b);
        }
    }

    help(slice);
    dbg!(&slice);

    for i in 1..slice.len() {
        help(&mut slice[i..]);
    }
}

pub fn max_heap_sort<T: Ord + Debug>(slice: &mut [T]) {
    if slice.len() <= 1 {
        return;
    }

    fn help<T: Ord + Debug>(slice: &mut [T]) {
        for i in (0..slice.len() / 2).rev() {
            heapify(slice, i, |a, b| a > b);
        }
    }

    help(slice);
    dbg!(&slice);

    for i in 1..slice.len() {
        help(&mut slice[i..]);
    }
}

pub fn heapify<T: Ord>(slice: &mut [T], idx: usize, comparator: fn(&T, &T) -> bool) {
    let len = slice.len();
    let (left, right) = (2 * idx + 1, 2 * idx + 2);

    if right < len && !comparator(&slice[idx], &slice[right]) {
        slice.swap(idx, right);
        heapify(slice, right, comparator);
    }

    if left < len && !comparator(&slice[idx], &slice[left]) {
        slice.swap(idx, left);
        heapify(slice, left, comparator);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_heap_sort() {
        let mut nums = vec![9, 7, 5, 11, 12, 2, 14, 3, 10, 6];
        println!("Before: {:?}", nums);
        let mut ans = vec![2, 3, 5, 6, 7, 9, 10, 11, 12, 14];

        min_heap_sort(&mut nums);
        println!("After: {:?}", nums);
        assert_eq!(nums, ans);

        max_heap_sort(&mut nums);
        println!("After: {:?}", nums);
        ans.reverse();
        assert_eq!(nums, ans);
    }
}
