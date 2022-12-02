use std::cmp::PartialOrd;

pub fn bubble_sort<T: PartialOrd>(items: &mut Vec<T>) {
    let len = items.len();
    if len <= 1 {
        return;
    }

    for i in 0..len {
        for j in i + 1..len {
            if items[j] < items[i] {
                items.swap(i, j)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::bubble_sort;

    #[test]
    fn t_bubble_sort() {
        let mut items = vec![7, 3, 5, 1, 9];
        bubble_sort(&mut items);
        assert_eq!(items, vec![1, 3, 5, 7, 9]);
    }
}
