use std::fmt::Debug;

// O(2*n^2)
fn palindroms<T: Eq + PartialEq + Debug>(slice: &[T]) -> Vec<&[T]> {
    if slice.is_empty() {
        return Vec::new();
    } else if slice.len() == 1 {
        return vec![slice];
    }

    let mut length = 0;
    let mut vec: Vec<&[T]> = Vec::new();

    for index in 0..slice.len() {
        // println!("~~~ {} {:?}", index, value);
        let (mut low, mut high) = (index, index);

        while slice[low] == slice[high] {
            if high - low + 1 > length {
                length = high - low + 1;
                vec.clear();
            }
            if high - low + 1 >= length {
                vec.push(&slice[low..=high]);
            }

            if low == 0 || high == slice.len() - 1 {
                break;
            }

            low -= 1;
            high += 1;
        }

        (low, high) = (index, index + 1);
        if high >= slice.len() {
            break;
        }

        while slice[low] == slice[high] {
            if high - low + 1 > length {
                length = high - low + 1;
                vec.clear();
            }
            if high - low + 1 >= length {
                vec.push(&slice[low..=high]);
            }

            if low == 0 || high == slice.len() - 1 {
                break;
            }

            low -= 1;
            high += 1;
        }
    }

    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_palindroms() {
        let vec = palindroms(&['a', 'b', 'a', 'a', 'b']);
        dbg!(&vec);

        assert_eq!(vec.len(), 1);
    }
}
