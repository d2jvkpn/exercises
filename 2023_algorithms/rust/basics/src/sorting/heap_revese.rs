use std::fmt::Debug;

pub struct Heap<T: Ord + Debug> {
    vec: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T: Ord + Debug> Heap<T> {
    pub fn new(size: usize, comparator: fn(&T, &T) -> bool) -> Self {
        Self { vec: Vec::with_capacity(size), comparator }
    }

    pub fn size(&self) -> usize {
        self.vec.len()
    }

    fn sort(&mut self) {
        for i in (0..self.vec.len() / 2).rev() {
            heapify(&mut self.vec, i, self.comparator);
        }
    }

    pub fn peak(&self) -> Option<&T> {
        match self.vec.len() {
            0 => None,
            _ => Some(&self.vec[0]),
        }
    }

    pub fn push(&mut self, value: T) {
        self.vec.push(value);
        self.sort();
    }

    pub fn pop(&mut self) -> Option<T> {
        let value = self.vec.pop()?;
        self.sort();
        Some(value)
    }
}

fn heapify<T: Ord>(slice: &mut [T], idx: usize, comparator: fn(&T, &T) -> bool) {
    let len = slice.len();
    let (left, right) = (2 * idx + 1, 2 * idx + 2);

    if right < len && !comparator(&slice[idx], &slice[right]) {
        slice.swap(idx, right);
        heapify(slice, right, comparator);
    }

    if left < len && !comparator(&slice[idx], &slice[left]) {
        slice.swap(idx, left);
        heapify(slice, left, comparator);
    }
}
