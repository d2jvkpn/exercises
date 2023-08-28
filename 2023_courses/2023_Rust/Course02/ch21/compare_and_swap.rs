fn main() {
    let mut vec = [5, 10, 1, 4, 11];
    let expected = [1, 4, 5, 10, 11];

    selection_sort(&mut vec);
    assert_eq!(vec, expected);

    let mut vec = [5, 10, 1, 4, 11];
    bubble_sort(&mut vec);
    assert_eq!(vec, expected);
}

// selection sort
// select the min value from the tail and swap it with the head
fn selection_sort<T: PartialOrd>(vec: &mut [T]) {
    let len = vec.len();
    if len < 2 {
        return;
    }

    for i in 0..len - 1 {
        let mut idx = i;
        for j in i + 1..len {
            if vec[j] < vec[idx] {
                idx = j;
            }
        }

        if idx != i {
            vec.swap(i, idx);
        }
    }
}

// iterate from i to len-1, swap k with k+1 if vec[k+1] < vec[k]
fn bubble_sort<T: PartialOrd>(vec: &mut [T]) {
    let len = vec.len();
    if len < 2 {
        return;
    }

    for i in 0..len {
        for j in (i..len - 1).rev() {
            if vec[j] > vec[j + 1] {
                vec.swap(j, j + 1);
            }
        }
    }
}
