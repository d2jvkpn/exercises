use std::fmt::Debug;

pub fn min_heap_sort<T: Ord + Debug>(slice: &mut [T]) {
    for i in (0..slice.len() / 2).rev() {
        heapify(slice, i, |a, b| a < b);
    }
    dbg!(&slice);

    for i in (1..slice.len()).rev() {
        slice.swap(0, i);
        heapify(&mut slice[..i], 0, |a, b| a < b);
    }
}

pub fn max_heap_sort<T: Ord + Debug>(slice: &mut [T]) {
    for i in (0..slice.len() / 2).rev() {
        heapify(slice, i, |a, b| a > b);
    }
    dbg!(&slice);

    for i in (1..slice.len()).rev() {
        slice.swap(0, i);
        heapify(&mut slice[..i], 0, |a, b| a > b);
    }
}

fn heapify<T: Ord + Debug>(slice: &mut [T], idx: usize, comparator: fn(&T, &T) -> bool) {
    let len = slice.len();
    let mut target = idx;
    let (left, right) = (2 * idx + 1, 2 * idx + 2);

    if left < len && comparator(&slice[target], &slice[left]) {
        target = left;
    }

    if right < len && comparator(&slice[target], &slice[right]) {
        target = right;
    }

    if target != idx {
        slice.swap(idx, target);
        heapify(slice, target, comparator);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_heap_sort() {
        let mut nums = vec![9, 7, 5, 11, 12, 2, 14, 3, 10, 6];
        println!("Before: {:?}", nums);

        min_heap_sort(&mut nums);
        println!("After: {:?}", nums);
        assert_eq!(nums, vec![2, 3, 5, 6, 7, 9, 10, 11, 12, 14]);

        max_heap_sort(&mut nums);
        println!("After: {:?}", nums);
        assert_eq!(nums, vec![14, 12, 11, 10, 9, 7, 6, 5, 3, 2]);
    }
}
