#![allow(dead_code)]

fn merge_sort<T: Copy + Ord>(slice: &mut [T]) {
    fn merge<T: Copy + PartialOrd>(s1: &[T], s2: &[T]) -> Vec<T> {
        let (mut i, mut j) = (0, 0);
        let mut result = Vec::with_capacity(s1.len() + s2.len());

        while i < s1.len() && j < s2.len() {
            if s1[i] < s2[j] {
                result.push(s1[i]);
                i += 1;
            } else {
                result.push(s2[j]);
                j += 1;
            }
        }

        result.extend_from_slice(&s1[i..]);
        result.extend_from_slice(&s2[j..]);
        result
    }

    if slice.len() < 2 {
        return;
    }
    let m = slice.len() / 2;

    merge_sort(&mut slice[..m]);
    merge_sort(&mut slice[m..]);

    let result = merge(&slice[..m], &slice[m..]);
    slice.copy_from_slice(&result);
}

fn quick_sort<T: Copy + Ord>(slice: &mut [T]) {
    if slice.len() < 2 {
        return;
    }

    let mut s1 = Vec::new();
    let mut s2 = Vec::new();
    let val = slice[0];

    slice[1..].iter().for_each(|&v| {
        if v <= val {
            s1.push(v);
        } else {
            s2.push(v);
        }
    });

    quick_sort(&mut s1);
    quick_sort(&mut s2);
    let mut result = Vec::with_capacity(slice.len());
    result.extend_from_slice(&s1);
    result.push(val);
    result.extend_from_slice(&s2);

    slice.copy_from_slice(&result);
}

#[cfg(test)]
mod tests {
    use super::*;

    static ARRAY_01: [i32; 9] = [14, 33, 10, 27, 19, 35, 42, 44, 18];
    static EXPECTED_01: &[i32] = &[10, 14, 18, 19, 27, 33, 35, 42, 44];

    #[test]
    fn t_vec() {
        let mut v1 = vec![1, 2, 3];
        let v2 = vec![4, 5, 6];

        v1.copy_from_slice(&v2);
        assert_eq!(v2[v2.len()..].len(), 0);
        assert_eq!(v1, v2);
    }

    #[test]
    fn t_merge_sort() {
        let mut vec = ARRAY_01.to_vec();

        merge_sort(&mut vec);
        assert_eq!(&vec, &EXPECTED_01);
    }

    #[test]
    fn t_quick_sort() {
        let mut vec = ARRAY_01.to_vec();
        quick_sort(&mut vec);
        assert_eq!(&vec, &EXPECTED_01);
    }
}
