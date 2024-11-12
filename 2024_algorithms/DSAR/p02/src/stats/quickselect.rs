fn partition<T: PartialOrd>(array: &mut [T], low: usize, high: usize) -> usize {
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

fn quickselect<T: PartialOrd>(array: &mut [T], low: usize, high: usize, k: usize) -> T {
    if low == high {
        return array[low];
    }

    let pivot_index = partition(array, low, high);

    if k == pivot_index {
        array[k]
    } else if k < pivot_index {
        quickselect(array, low, pivot_index - 1, k)
    } else {
        quickselect(array, pivot_index + 1, high, k)
    }
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn t_quick() {
    	let mut data = vec![3, 5, 1, 9, 7, 6, 2, 8, 4];
    	let k = 4;  // Looking for the 5th smallest element, 0-based index
    	let result = quickselect(&mut data, 0, data.len() - 1, k);
    	println!("The {}-th smallest element is: {}", k + 1, result);
	}
}
