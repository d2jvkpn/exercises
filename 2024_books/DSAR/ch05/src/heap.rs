use std::fmt::{self, Debug};

pub struct Heap<T: PartialOrd + Debug> {
    data: Vec<T>,
    ascending: bool,
    comparator: fn(&T, &T) -> bool,
}

pub fn build_heap<T: PartialOrd>(slice: &mut [T], compare: fn(&T, &T) -> bool) {
    for i in (0..=slice.len() / 2).rev() {
        heapify(slice, i, compare);
    }
}

pub fn heapify<T: PartialOrd>(slice: &mut [T], index: usize, compare: fn(&T, &T) -> bool) {
    let mut next = index;

    let left = 2 * index + 1;
    if left < slice.len() && !compare(&slice[next], &slice[left]) {
        next = left;
    }

    let right = 2 * index + 2;
    if right < slice.len() && !compare(&slice[next], &slice[right]) {
        next = right;
    }

    if next != index {
        slice.swap(index, next);
        heapify(slice, next, compare);
    }
}

impl<T: PartialOrd + Debug> Heap<T> {
    pub fn new(size: usize, ascending: bool) -> Self {
        Self {
            data: Vec::with_capacity(size),
            ascending,
            comparator: if ascending { |a, b| a < b } else { |a, b| a > b },
        }
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn cap(&self) -> usize {
        self.data.capacity()
    }

    pub fn is_empty(&self) -> bool {
        self.data.len() == 0
    }

    #[allow(dead_code)]
    pub fn ascending(&self) -> bool {
        self.ascending
    }

    #[allow(dead_code)]
    fn compare(&self, a: usize, b: usize) -> bool {
        (self.comparator)(&self.data[a], &self.data[b])
    }

    pub fn peek(&self) -> Option<&T> {
        self.data.first()
    }

    pub fn push(&mut self, value: T) -> &mut Self {
        self.data.push(value);
        build_heap(&mut self.data, self.comparator);

        self
    }

    pub fn pop(&mut self) -> Option<T> {
        let size = self.size();

        if size == 0 {
            return None;
        }

        self.data.swap(0, size - 1);
        let ans = self.data.pop();
        build_heap(&mut self.data, self.comparator);

        ans
    }

    pub fn min_heap(items: Vec<T>) -> Self {
        let mut heap = Self { data: items, ascending: true, comparator: |a, b| a < b };

        build_heap(&mut heap.data, heap.comparator);

        heap
    }

    pub fn max_heap(items: Vec<T>) -> Self {
        let mut heap = Self { data: items, ascending: false, comparator: |a, b| a > b };

        build_heap(&mut heap.data, heap.comparator);

        heap
    }
}

impl<T: PartialOrd + Debug> Debug for Heap<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let typ = if self.ascending { "min_heap" } else { "max_heap" };
        write!(f, "{typ}={:?}", self.data)
    }
}

impl<T: PartialOrd + Debug> From<Heap<T>> for Vec<T> {
    fn from(heap: Heap<T>) -> Vec<T> {
        /*
        let mut ans = Vec::with_capacity(self.size());

        while let Some(v) = self.pop() {
            ans.push(v);
        }

        ans
        */

        let high = heap.size() - 1;
        let compare = heap.comparator;
        let mut ans = heap.data;

        for i in 1..ans.len() {
            build_heap(&mut ans[i..], compare);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_heap() {
        let nums = vec![9, 7, 5, 11, 12, 2, 14, 3, 10, 6];

        // 1.
        let mut heap = Heap::max_heap(nums.clone());
        println!("==> heap: {:?}", heap);

        assert_eq!(heap.peek(), Some(&14));
        assert_eq!(heap.pop(), Some(14));
        assert_eq!(heap.pop(), Some(12));

        println!("==> heap: {:?}", heap);

        // 2.
        let heap = Heap::max_heap(nums.clone());

        let output: Vec<_> = heap.into();
        assert_eq!(output, vec![14, 12, 11, 10, 9, 7, 6, 5, 3, 2]);

        // 3.
        let heap = Heap::min_heap(nums.clone());

        let output: Vec<_> = heap.into();
        assert_eq!(output, vec![2, 3, 5, 6, 7, 9, 10, 11, 12, 14]);
    }
}
