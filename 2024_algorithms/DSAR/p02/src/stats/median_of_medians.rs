fn median_of_medians<T: Ord + Copy>(array: &[T], k: usize) -> Option<T> {
    let len = array.len();

    if len < k {
        return None;
    }

    if len <= 5 {
        let mut sorted_array = array.to_vec();
        sorted_array.sort_unstable();
        return Some(sorted_array[k]);
    }

    let mut medians = Vec::new();
    for chunk in array.chunks(5) {
        let mut sorted_chunk = chunk.to_vec();
        sorted_chunk.sort_unstable();
        medians.push(sorted_chunk[2]);
    }

    let pivot = median_of_medians(&medians, medians.len() / 2)?;

    let (left, right): (Vec<_>, Vec<_>) = array.iter().partition(|&&x| x < pivot);

    if k < left.len() {
        median_of_medians(&left, k)
    } else if k >= len - right.len() {
        median_of_medians(&right, k - (len - right.len()))
    } else {
        Some(pivot)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_median_of_medians() {
        let data = [3, 5, 1, 9, 7, 6, 2, 8, 4];
        let k = 10; // Looking for the 5th smallest element (0-based index)
        let result = median_of_medians(&data, k);

        println!("The {}-th smallest element is: {:?}", k + 1, result);

        assert_eq!(median_of_medians(&data, 0), Some(1));
        assert_eq!(median_of_medians(&data, 4), Some(5));
        assert_eq!(median_of_medians(&data, 10), None);
    }
}
