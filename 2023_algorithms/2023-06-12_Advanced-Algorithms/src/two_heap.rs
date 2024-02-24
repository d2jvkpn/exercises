#![allow(dead_code)]

use basics::structs::heap_priority_queue::Heap;
use std::fmt::Debug;

// TODO
pub fn get_median<T: Ord + Debug + Copy>(slice: &[T]) -> T {
    let num = (slice.len() + 1) / 2;
    let mut _max_heap: Heap<T> = Heap::min_heap(num);
    let mut _min_heap: Heap<T> = Heap::min_heap(num);

    slice.iter().for_each(|_v| todo!());

    todo!()
}
