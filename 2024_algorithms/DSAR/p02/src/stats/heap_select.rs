use std::{cmp::Reverse, collections::BinaryHeap};

fn heap_select<T: Ord + Clone>(array: &[T], k: usize) -> T {
    // Ensure k is within the bounds of the array
    assert!(k > 0 && k <= array.len());

    let mut min_heap = BinaryHeap::with_capacity(k);

    // Build the heap with the first k elements
    for value in array.iter().take(k) {
        min_heap.push(Reverse(value.clone()));
    }

    // Process the rest of the array
    for value in array.iter().skip(k) {
        if *value > min_heap.peek().unwrap().0 {
            min_heap.pop();
            min_heap.push(Reverse(value.clone()));
        }
    }

    // Return the k-th smallest element
    min_heap.peek().unwrap().0.clone()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn t_heap_sort() {
        let data = vec![3, 5, 1, 9, 7, 6, 2, 8, 4];
        let k = 4; // Looking for the 5th smallest element, 0-based index
        let result = heap_select(&data, k);
        println!("The {}-th smallest element is: {}", k + 1, result);
    }
}
