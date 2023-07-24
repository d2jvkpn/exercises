use basics::sorting::heap_dynamic::Heap;
use std::fmt::Debug;

// TODO
pub fn get_median<T: Ord + Debug + Copy>(slice: &[T]) -> T {
    let num = (slice.len() + 1) / 2;
    let mut max_heap: Heap<T> = Heap::min_heap(num);
    let mut min_heap: Heap<T> = Heap::min_heap(num);

    slice.iter().for_each(|v| todo!());

    todo!()
}
