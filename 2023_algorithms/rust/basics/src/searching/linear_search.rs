fn linear_search<T: PartialOrd + PartialEq + Copy>(slice: &[T], target: T) -> Option<usize> {
    for (i, v) in slice.iter().enumerate() {
        if v == &target {
            return Some(i);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    static ARRARY_01: [i32; 10] = [17, 4, 31, 9, 10, 14, 1, 20, 27, 7];

    #[test]
    fn t_linear_search() {
        assert_eq!(linear_search(&ARRARY_01, 0), None);
        assert_eq!(linear_search(&ARRARY_01, 9), Some(3));
    }
}
