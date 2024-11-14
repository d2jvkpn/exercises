pub fn partition<T: PartialOrd>(array: &mut [T], low: usize, high: usize) -> usize {
    let pivot = high;
    let mut i = low;

    for j in low..high {
        if array[j] <= array[pivot] {
            array.swap(i, j);
            i += 1;
        }
    }

    array.swap(i, high);
    i
}

fn _quickselect<T: PartialOrd>(array: &mut [T], low: usize, high: usize, k: usize) -> &T {
    if low == high {
        return &array[low];
    }

    let pivot_index = partition(array, low, high);

    if k == pivot_index {
        &array[k]
    } else if k < pivot_index {
        _quickselect(array, low, pivot_index - 1, k)
    } else {
        _quickselect(array, pivot_index + 1, high, k)
    }
}

fn quickselect<T: PartialOrd>(array: &mut [T], k: usize) -> Option<&T> {
    if k == 0 || k > array.len() {
        return None;
    }

    Some(_quickselect(array, 0, array.len() - 1, k - 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_quickselect() {
        let mut data = vec![3, 5, 1, 9, 7, 6, 2, 8, 4];

        // Looking for the 5th smallest element, 1-based index
        assert_eq!(quickselect(&mut data, 4), Some(&4));

        assert_eq!(quickselect(&mut data, 3), Some(&3));

        assert!(quickselect(&mut data, 0).is_none());
        assert_eq!(quickselect(&mut data, 1), Some(&1));

        assert_eq!(quickselect(&mut data, 9), Some(&9));
        assert!(quickselect(&mut data, 10).is_none());
    }
}
