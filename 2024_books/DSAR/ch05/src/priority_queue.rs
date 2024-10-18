use std::fmt::Debug;

#[derive(Debug)]
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
    let left = 2 * index + 1;
    let right = 2 * index + 2;
    let mut next = index;

    if left < slice.len() && !compare(&slice[next], &slice[left]) {
        next = left;
    }

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

    pub fn from_vector(items: Vec<T>, ascending: bool) -> Self {
        let mut heap = Self {
            data: items,
            ascending,
            comparator: if ascending { |a, b| a < b } else { |a, b| a > b },
        };

        build_heap(&mut heap.data, heap.comparator);

        heap
    }

    pub fn into_ordered_vector(mut self) -> Vec<T> {
        /*
        let mut ans = Vec::with_capacity(self.size());

        while let Some(v) = self.pop() {
            ans.push(v);
        }

        ans
        */
        let high = self.size() - 1;
        let mut ans = self.data;

        for i in (1..=high).rev() {
            ans.swap(0, i);
            build_heap(&mut ans[..i], self.comparator);
        }

        ans.reverse();
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_heap() {
        //
        let nums = vec![9, 7, 5, 11, 12, 2, 14, 3, 10, 6];

        let mut heap = Heap::from_vector(nums.clone(), false);
        println!("==> heap: {:?}", heap);

        assert_eq!(heap.peek(), Some(&14));
        assert_eq!(heap.pop(), Some(14));
        assert_eq!(heap.pop(), Some(12));

        println!("==> heap: {:?}", heap);

        //
        let heap = Heap::from_vector(nums.clone(), false);

        let sorted = vec![14, 12, 11, 10, 9, 7, 6, 5, 3, 2];
        assert_eq!(heap.into_ordered_vector(), sorted);

        //
        let heap = Heap::from_vector(nums.clone(), true);

        let sorted = vec![2, 3, 5, 6, 7, 9, 10, 11, 12, 14];
        assert_eq!(heap.into_ordered_vector(), sorted);
    }
}
