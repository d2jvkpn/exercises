fn sort_positive_negative(slice: &mut [isize]) {
    slice.sort_by(|a, b| if a * b < 0 || a < &0 { b.cmp(a) } else { a.cmp(b) });
}

#[cfg(test)]
mod tests {
    use super::*;

    static ARRAY: [isize; 8] = [4, 1, 3, 2, -3, -1, -4, -2];
    static EXPECTED: [isize; 8] = [1, 2, 3, 4, -1, -2, -3, -4];

    #[test]
    fn t_sort_positive_negative() {
        let mut vec = ARRAY.to_vec();

        sort_positive_negative(&mut vec);
        assert_eq!(vec, &EXPECTED);
    }
}
