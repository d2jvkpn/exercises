pub struct Heap {
    count: usize,
    items: Vec<i64>,
}

// min heap
impl Heap {
    pub fn new() -> Self {
        Self { count: 0, items: vec![0] }
    }

    pub fn new_with_cap(cap: usize) -> Self {
        let mut items = Vec::with_capacity(cap + 1);
        items.push(0);
        Self { count: 0, items }
    }

    pub fn from_slice(slice: &[i64]) -> Self {
        let mut heap = Self::new_with_cap(slice.len());
        slice.iter().for_each(|v| heap.push(*v));
        heap
    }

    pub fn size(&self) -> usize {
        self.count
    }

    pub fn print(&self) {
        println!("items: {:?}\n", &self.items[1..]);
    }

    pub fn push(&mut self, value: i64) {
        self.count += 1;
        self.items.push(value);
        let mut idx = self.count;

        loop {
            let pdx = idx / 2;
            if pdx == 0 {
                break;
            }

            if &self.items[idx] < &self.items[pdx] {
                self.items.swap(idx, pdx);
            }

            idx = pdx;
        }
    }

    pub fn pop(&mut self) -> Option<i64> {
        if self.count == 0 {
            return None;
        }

        self.count -= 1;
        let option = Some(self.items.swap_remove(1));
        if self.count == 0 {
            return option;
        }

        let mut idx = 1;
        loop {
            let lidx = idx * 2;
            if lidx > self.count {
                break;
            }

            if lidx + 1 > self.count {
                if self.items[idx] > self.items[lidx] {
                    self.items.swap(idx, lidx);
                }
                break;
            }

            let left = self.items[lidx];
            let right = self.items[lidx + 1];
            let min = if left < right { lidx } else { lidx + 1 };
            self.items.swap(idx, min);
            idx = min;
        }

        option
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_vec() {
        let mut vec = vec!["foo", "bar", "baz", "qux"];

        assert_eq!(vec.swap_remove(1), "bar");
        assert_eq!(vec, ["foo", "qux", "baz"]);
        assert_eq!(vec.capacity(), 4);

        vec.push("xx");
        assert_eq!(vec.capacity(), 4);
    }

    #[test]
    fn t_heap() {
        let mut heap = Heap::from_slice(&vec![2, 8, 5, 3, 9, 1, 11]);
        heap.print();
        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.pop(), Some(2));
        assert_eq!(heap.pop(), Some(3));
        assert_eq!(heap.pop(), Some(5));
        heap.print();
        assert_eq!(heap.pop(), Some(8));
        heap.print();
        assert_eq!(heap.pop(), Some(9));
        assert_eq!(heap.pop(), Some(11));
        assert_eq!(heap.pop(), None);
    }
}
