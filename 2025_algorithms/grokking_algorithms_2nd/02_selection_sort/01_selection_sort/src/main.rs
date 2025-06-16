fn selection_sort<T: PartialOrd + Copy>(list: &mut [T], asc: bool) {
    let cmp_fn = if asc { |a, b| a < b } else { |a, b| a > b };

    for i in 0..list.len() {
        for j in (i + 1)..list.len() {
            if !cmp_fn(list[i], list[j]) {
                list.swap(i, j);
            }
        }
    }
}

fn sorted<T: PartialOrd + Copy>(list: &[T], asc: bool) -> bool {
    if list.len() < 2 {
        return true;
    }

    let cmp_fn = if asc { |a, b| a < b } else { |a, b| a > b };
    for i in 0..(list.len() - 1) {
        if !cmp_fn(list[i], list[i + 1]) {
            return false;
        }
    }

    true
}

fn main() {
    let list = &mut [156, 141, 35, 94, 88, 61, 111];
    selection_sort(list, true);
    assert!(sorted(list, true));
    println!("{list:?}")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sort_unsigned_list() {
        let mut list: [u8; 7] = [156, 141, 35, 94, 88, 61, 111];
        selection_sort(&mut list[..], true);
        assert!(sorted(&list[..], true));
    }

    #[test]
    fn sort_signed_list() {
        let mut list: [i32; 10] = [75, 85, -26, 61, 20, -40, -72, 30, -27, 58];

        selection_sort(&mut list[..], true);
        assert!(sorted(&list[..], true));
    }

    #[test]
    fn sort_strings() {
        let mut list: [&str; 7] = [
            "Radiohead",
            "Kishore Kumar",
            "The Black Keys",
            "Neutral Milk Hotel",
            "Beck",
            "The Strokes",
            "Wilco",
        ];

        let list_sorted: [&str; 7] = [
            "Wilco",
            "The Strokes",
            "The Black Keys",
            "Radiohead",
            "Neutral Milk Hotel",
            "Kishore Kumar",
            "Beck",
        ];

        selection_sort(&mut list[..], false);
        assert!(sorted(&list[..], false));
        assert_eq!(list, list_sorted);
    }

    #[test]
    fn sorts_an_empty_list() {
        let mut list: [u8; 0] = [];
        selection_sort(&mut list[..], true);
        assert!(sorted(&list[..], true));
    }
}
