use std::fmt::Debug;

#[derive(Debug)]
pub struct Heap<T: Ord + Debug> {
    data: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T: Ord + Debug> Heap<T> {
    pub fn new(size: usize, comparator: fn(&T, &T) -> bool) -> Self {
        Self { data: Vec::with_capacity(size), comparator }
    }

    pub fn min_heap(size: usize) -> Self {
        Self { data: Vec::with_capacity(size), comparator: |a, b| a < b }
    }

    pub fn max_heap(size: usize) -> Self {
        Self { data: Vec::with_capacity(size), comparator: |a, b| a > b }
    }

    pub fn empty(&self) -> bool {
        self.data.len() == 0
    }

    pub fn top(&self) -> Option<&T> {
        self.data.last()
    }

    fn compare(&self, parent: usize, child: usize) -> bool {
        (self.comparator)(&self.data[parent], &self.data[child])
    }

    pub fn push(&mut self, value: T) -> &mut Self {
        self.data.push(value);
        self.heapify_up(self.data.len() - 1);
        self
    }

    pub fn pop(&mut self) -> Option<T> {
        let size = self.data.len();
        if size == 0 {
            return None;
        }

        self.data.swap(0, size - 1);
        let ans = self.data.pop();
        self.heapify_down(0);
        ans
    }

    pub fn heapify_up(&mut self, index: usize) {
        // dbg!(&index);
        if index < 1 || index + 1 > self.data.len() {
            return;
        }

        let parent = (index - 1) / 2;
        if !self.compare(parent, index) {
            self.data.swap(parent, index);
            self.heapify_up(parent);
        }
    }

    pub fn heapify_down(&mut self, index: usize) {
        // dbg!(&index);
        let mut parent = index;
        let (left, right) = (2 * index + 1, 2 * index + 2);

        if left + 1 > self.data.len() {
            return;
        }

        if !self.compare(parent, left) {
            parent = left;
        }

        if right < self.data.len() && !self.compare(parent, right) {
            parent = right;
        }

        if parent != index {
            self.data.swap(parent, index);
            self.heapify_down(parent);
        }
    }

    pub fn from_vector(mut items: Vec<T>, comparator: fn(&T, &T) -> bool) -> Self {
        let mut heap = Self::new(items.len(), comparator);

        while let Some(val) = items.pop() {
            heap.push(val);
        }

        heap
    }

    pub fn into_vector(mut self) -> Vec<T> {
        let mut items = Vec::with_capacity(self.data.len());

        while let Some(val) = self.pop() {
            items.push(val);
        }

        items
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_heap_pq() {
        let nums = vec![9, 7, 5, 11, 12, 2, 14, 3, 10, 6];
        let ans = vec![2, 3, 5, 6, 7, 9, 10, 11, 12, 14];

        //
        let heap = Heap::from_vector(nums.clone(), |a, b| a < b);
        dbg!(&heap);
        assert_eq!(heap.into_vector(), ans);

        //
        let heap = Heap::from_vector(nums.clone(), |a, b| a > b);
        dbg!(&heap);

        let mut rev = ans.clone();
        rev.reverse();
        assert_eq!(heap.into_vector(), rev);
    }
}
