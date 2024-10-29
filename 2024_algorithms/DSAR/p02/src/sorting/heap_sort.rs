use std::fmt::Debug;

pub fn heap_sort<T: PartialOrd + Debug>(slice: &mut [T]) {
    if slice.len() < 2 {
        return
    }
    println!("==> heap_sort.1. {:?}", &slice);

    build_max_heap(slice);
    println!("==> heap_sort.2. {:?}", &slice);

    let (low, high) = (0, slice.len() - 1);

    for i in (low + 1..=high).rev() {
        slice.swap(low, i); // swaped the biggest value(head) to the tail
        max_heapify(slice, low, low, i - 1);
    }
    println!("==> heap_sort.3. {:?}", &slice);
}

// all parants are greather than children
fn build_max_heap<T: PartialOrd + Debug>(slice: &mut [T]) {
    let (low, high) = (0, slice.len() - 1);

    let mut index = (high - low) / 2;

    while index >= low {
        max_heapify(slice, index, low, high);
        if index == 0 {
            break;
        }
        index -= 1;
    }
}

fn max_heapify<T: PartialOrd>(slice: &mut [T], index: usize, low: usize, high: usize) {
    let left = 2 * index + 1;
    let right = 2 * index + 2;
    let mut next = index;

    if left <= high && slice[left] > slice[next] {
        next = left;
    }

    if right <= high && slice[right] > slice[next] {
        next = right;
    }

    if next != index {
        slice.swap(index, next);
        max_heapify(slice, next, low, high);
    }
}
