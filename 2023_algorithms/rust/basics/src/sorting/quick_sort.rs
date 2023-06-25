pub fn partition<T: Copy + Ord>(arr: &mut [T], low: usize, high: usize) -> usize {
    let mut index = low;
    let pivot = arr[high];

    for i in low..high {
        if arr[i] <= pivot {
            if index != i {
                arr.swap(index, i);
            }
            index += 1;
        }
    }

    arr.swap(index, high);
    index
}

fn quick_sort_range<T: Copy + Ord>(arr: &mut [T], low: usize, high: usize) {
    if arr.len() <= 1 {
        return;
    }

    if low < high {
        let pivot_index = partition(arr, low, high);
        quick_sort_range(arr, low, pivot_index - 1);
        quick_sort_range(arr, pivot_index + 1, high);
    }
}

pub fn quick_sort<T: Copy + Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    let high = arr.len() - 1;
    quick_sort_range(arr, 0, high);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_quick_sort() {
        let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
        quick_sort(&mut arr);
        assert_eq!(arr, vec![1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 9])
    }
}
