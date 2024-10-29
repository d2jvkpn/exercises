#![allow(unused_imports)]

fn find_min_max<T: PartialOrd + Copy>(array: &[T]) -> Option<(T, T)> {
    if array.is_empty() {
        return None;
    }

    let mut min_value = array[0];
    let mut max_value = array[0];

    for &item in array.iter() {
        if item < min_value {
            min_value = item;
        }

        if item > max_value {
            max_value = item;
        }
    }

    Some((min_value, max_value))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_find_min_max() {
        let numbers = [3, 5, 1, 9, 7];
        let ans = find_min_max(&numbers);

        assert_eq!(ans, Some((1, 9)));
    }
}
