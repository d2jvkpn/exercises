use std::fmt::Debug;

#[derive(Debug)]
pub struct Heap<T: Ord + Debug> {
    vec: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T: Ord + Debug> Heap<T> {
    pub fn new(size: usize, comparator: fn(&T, &T) -> bool) -> Self {
        Self { vec: Vec::with_capacity(size), comparator }
    }

    pub fn from_vec(mut vec: Vec<T>, comparator: fn(&T, &T) -> bool) -> Self {
        Self::push_slice(&mut vec, comparator);
        Self { vec, comparator }
    }

    pub fn min_heap(size: usize) -> Self {
        Self { vec: Vec::with_capacity(size), comparator: |a, b| a < b }
    }

    pub fn max_heap(size: usize) -> Self {
        Self { vec: Vec::with_capacity(size), comparator: |a, b| a > b }
    }

    pub fn size(&self) -> usize {
        self.vec.len()
    }

    fn push_slice(vec: &mut [T], comparator: fn(&T, &T) -> bool) {
        for i in vec.len() / 2..vec.len() {
            // dbg!(&i);
            heapify(vec, i, comparator);
        }
    }

    pub fn peak(&self) -> Option<&T> {
        self.vec.last()
    }

    pub fn compare(&self, value: &T) -> Option<bool> {
        let top = self.vec.last()?;
        Some((self.comparator)(top, value))
    }

    pub fn push(&mut self, value: T) {
        self.vec.push(value);
        Self::push_slice(&mut self.vec, self.comparator);
    }

    pub fn pop(&mut self) -> Option<T> {
        let value = self.vec.pop()?;
        Self::push_slice(&mut self.vec, self.comparator);
        Some(value)
    }

    pub fn sort(mut self) -> Vec<T> {
        if self.size() <= 1 {
            return self.vec;
        }

        let m = self.size() - 1;
        for i in 1..self.size() {
            self.vec.swap(i - 1, m);
            Self::push_slice(&mut self.vec[i..], self.comparator);
            // dbg!((i, &self.vec));
        }

        self.vec
    }
}

pub fn heapify<T: Ord>(slice: &mut [T], idx: usize, comparator: fn(&T, &T) -> bool) {
    let m = slice.len() - 1;
    // dbg!(&idx);

    let (left, right) = match idx {
        _ if m >= 2 * (m - idx) + 2 => (m - 2 * (m - idx) - 1, m - 2 * (m - idx) - 2),
        _ if m == 2 * (m - idx) + 1 => (m - 2 * (m - idx) - 1, m),
        _ => return,
    };

    if right < idx && !comparator(&slice[idx], &slice[right]) {
        slice.swap(idx, right);
        heapify(slice, right, comparator);
    }

    if left < idx && !comparator(&slice[idx], &slice[left]) {
        slice.swap(idx, left);
        heapify(slice, left, comparator);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_heap() {
        let nums = vec![9, 7, 5, 11, 12, 2, 14, 3, 10, 6];
        let ans = vec![2, 3, 5, 6, 7, 9, 10, 11, 12, 14];

        //
        let mut rev = ans.clone();
        rev.reverse();
        let mut heap = Heap::new(nums.len(), |a, b| a < b);
        nums.iter().for_each(|v| heap.push(*v));

        dbg!(&heap);
        while let Some(v) = heap.pop() {
            assert_eq!(Some(v), rev.pop());
        }

        //
        let heap = Heap::from_vec(nums.clone(), |a, b| a < b);
        assert_eq!(heap.sort(), ans);

        //
        let mut rev = ans.clone();
        rev.reverse();
        let heap = Heap::from_vec(nums.clone(), |a, b| a > b);

        dbg!(&heap);
        assert_eq!(heap.sort(), rev);
    }
}
