fn binary_search<T: PartialOrd + PartialEq + Copy + std::fmt::Debug>(
    slice: &[T],
    target: T,
) -> Option<usize> {
    // slice was sorted
    if slice.len() == 0 {
        return None;
    }

    let (mut x1, mut x2) = (0, slice.len() - 1);

    loop {
        let m = (x1 + x2) / 2;
        let val = slice[m];

        if target == val {
            return Some(m);
        } else if x1 + 1 == x2 {
            return if target == slice[x2] { Some(x2) } else { None };
        } else if target < val {
            x2 = m;
        } else {
            x1 = m;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static ARRARY_01: [i32; 10] = [1, 4, 7, 9, 10, 14, 17, 20, 27, 31];
    static SEARCH_ITEMS: [(i32, Option<usize>); 3] = [(11, None), (27, Some(8)), (14, Some(5))];

    #[test]
    fn t_binary_search() {
        for (t, v) in SEARCH_ITEMS {
            assert_eq!(binary_search(&ARRARY_01, t), v);
        }

        // assert_eq!(binary_search(&ARRARY_01, 1), Some(0));
        // assert_eq!(binary_search(&ARRARY_01, 31), Some(9));
    }
}
