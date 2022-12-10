#![allow(dead_code)]

// use std::cmp::PartialOrd;

pub fn bubble_sort<T: PartialOrd>(items: &mut Vec<T>) {
    let len = items.len();
    if len <= 1 {
        return;
    }

    for i in (1..len).rev() {
        for j in 0..i {
            if items[j] > items[j + 1] {
                items.swap(j, j + 1)
            }
        }
    }
}

pub fn select_sort<T: PartialOrd>(items: &mut Vec<T>) {
    let len = items.len();
    if len <= 1 {
        return;
    }

    for i in 0..len {
        for j in i + 1..len {
            if items[i] > items[j] {
                items.swap(i, j)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_bubble_sort() {
        let mut items = vec![7, 3, 5, 1, 9];
        bubble_sort(&mut items);
        assert_eq!(items, vec![1, 3, 5, 7, 9]);
    }

    #[test]
    fn t_select_sort() {
        let mut items = vec![7, 3, 5, 1, 9];
        select_sort(&mut items);
        assert_eq!(items, vec![1, 3, 5, 7, 9]);
    }
}
