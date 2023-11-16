fn bubble_sort<T: PartialOrd>(slice: &mut [T]) {
    let mut index = slice.len() - 1;
    let mut next;

    loop {
        next = false;

        for i in 0..index {
            if slice[i] > slice[i + 1] {
                slice.swap(i, i + 1);
                next = true;
            }
        }

        if !next {
            break;
        }
        index -= 1;
    }
}

fn selection_sort<T: PartialOrd>(slice: &mut [T]) {
    for i in 0..slice.len() {
        let mut z = i;

        for j in i + 1..slice.len() {
            if slice[j] < slice[z] {
                z = j
            }
        }

        if z != i {
            slice.swap(i, z);
        }
    }
}

fn insertion_sort<T: PartialOrd>(slice: &mut [T]) {
    for i in 0..slice.len() {
        for j in (0..i).rev() {
            if slice[j] > slice[j + 1] {
                slice.swap(j, j + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static ARRAY_01: [i32; 9] = [14, 33, 10, 27, 19, 35, 42, 44, 18];
    static EXPECTED_01: &[i32] = &[10, 14, 18, 19, 27, 33, 35, 42, 44];

    #[test]
    fn t_selection_sort() {
        let mut ans = ARRAY_01.to_vec();

        selection_sort(&mut ans);
        assert_eq!(&ans, EXPECTED_01);
    }

    #[test]
    fn t_bubble_sort() {
        let mut ans = ARRAY_01.to_vec();

        bubble_sort(&mut ans);
        assert_eq!(&ans, EXPECTED_01);
    }

    #[test]
    fn t_insertion_sort() {
        let mut ans = ARRAY_01.to_vec();

        bubble_sort(&mut ans);
        assert_eq!(&ans, EXPECTED_01);
    }
}
