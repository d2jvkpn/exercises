use super::quick_sort::partition;

fn quick_select_range<T: Copy + Ord>(arr: &mut [T], k: usize, low: usize, high: usize) -> usize {
    if arr.len() <= 1 {
        return high;
    }

    if low >= high {
        return high;
    }

    let pivot = partition(arr, low, high);
    if pivot > k {
        return quick_select_range(arr, k, low, pivot - 1);
    } else if pivot < k {
        return quick_select_range(arr, k, pivot + 1, high);
    } else {
        return pivot;
    }
}

fn quick_select<T: Copy + Ord>(arr: &mut [T], k: usize) -> Option<usize> {
    if arr.len() == 0 || k == 0 {
        return None;
    } else if arr.len() < k {
        return Some(arr.len() - 1);
    }

    let ans = quick_select_range(arr, k, 0, arr.len() - 1);
    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::super::quick_sort::quick_sort;
    use super::*;

    #[test]
    fn t_quick_select() {
        let k = 5;
        let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
        let index = quick_select(&mut arr, k).unwrap();

        assert_eq!(index, k);

        quick_sort(&mut arr[..index]);
        assert_eq!(arr[..index], vec![1, 1, 2, 3, 3])
    }
}
