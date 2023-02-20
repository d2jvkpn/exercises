fn palindrom(arr: &Vec<i32>) -> bool {
    fn recursion(arr: &Vec<i32>, start: usize, end: usize) -> bool {
        if arr[start] != arr[end] {
            return false;
        }

        if end - start < 1 {
            return true;
        }

        recursion(arr, start + 1, end - 1)
    }

    recursion(arr, 0, arr.len() - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_palindrom() {
        assert!(!palindrom(&vec![1, 2, 3]));
        assert!(palindrom(&vec![1, 2, 1]));
    }
}
