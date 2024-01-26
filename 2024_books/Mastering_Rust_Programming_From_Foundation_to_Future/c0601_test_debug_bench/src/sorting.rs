#![allow(dead_code)]

pub fn bubble<T: PartialOrd>(slice: &mut [T]) {
    let len = slice.len();
    let mut swaped;

    for i in 1..len {
        swaped = false;
        for j in i..len {
            if slice[j - 1] > slice[j] {
                swaped = true;
                slice.swap(j - 1, j);
            }
        }

        if !swaped {
            break;
        }
    }
}

pub fn selection<T: PartialOrd>(slice: &mut [T]) {
    let len = slice.len();
    let mut low;

    for i in 0..len {
        low = i;
        for j in (i + 1)..len {
            if slice[j] < slice[low] {
                low = j;
            }
        }

        slice.swap(i, low);
    }
}

pub fn insertion<T: PartialOrd>(slice: &mut [T]) {
    for i in 0..slice.len() {
        for j in (0..i).rev() {
            if slice[j] > slice[j + 1] {
                slice.swap(j, j + 1);
            }
        }
    }
}
