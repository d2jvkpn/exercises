use std::cmp::Ordering;

pub fn binary_search<T: Ord>(slice: &[T], target: T) -> Option<usize> {
    let mut left = 0;
    let mut right = slice.len();

    while left < right {
        let mid = left + (right - left) / 2;

        match slice[mid].cmp(&target) {
            Ordering::Equal => return Some(mid),
            Ordering::Less => left = mid + 1,
            Ordering::Greater => right = mid,
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_binary_search() {
        println!("Hello, world!");

        let arr = vec![1, 2, 3, 5, 6, 10, 42];

        let ans = binary_search(&arr, 9);
        println!("==> {ans:?}");

        let ans = binary_search(&arr, 5);
        println!("==> {ans:?}");
    }
}
