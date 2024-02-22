pub fn quick_sort<T: Copy + Ord>(arr: &mut [T]) {
    fn partition<T: Copy + Ord>(arr: &mut [T], low: usize, high: usize) -> usize {
        let mut pivot = low;
        let value = arr[high];

        for i in low..high {
            if arr[i] <= value {
                if pivot != i {
                    arr.swap(pivot, i);
                }
                pivot += 1;
            }
        }

        arr.swap(pivot, high);
        pivot
    }

    fn qs<T: Copy + Ord>(arr: &mut [T], low: usize, high: usize) {
        if low >= high {
            return;
        }

        let pivot = partition(arr, low, high);

        if low + 1 < pivot {
            qs(arr, low, pivot - 1);
        }

        if pivot + 1 < high {
            qs(arr, pivot + 1, high);
        }
    }

    if arr.len() < 2 {
        return;
    }

    qs(arr, 0, arr.len() - 1);
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
