#![allow(dead_code)]

fn main() {
    // println!("Hello, wrold!");
    let mut deque = Deque::new(5);

    deque.add_rear(1).unwrap();
    deque.add_rear(2).unwrap();
    deque.add_rear(3).unwrap();

    assert!(deque.add_rear(4).is_ok()); // [Some(1), Some(2), Some(3), Some(4), None]
    assert_eq!(deque.size(), 4);

    assert_eq!(deque.pop_rear(), Some(4)); // [Some(1), Some(2), Some(3), None, None]
    assert_eq!(deque.pop_front(), Some(1)); // [None, Some(2), Some(3), None, None]

    assert!(deque.add_front(11).is_ok()); // [Some(11), Some(2), Some(3), None, None]
    assert!(deque.add_front(12).is_ok()); // [Some(11), Some(2), Some(3), None, Some(12)]
    println!("{:?}", deque);

    assert!(deque.add_rear(14).is_ok()); // [Some(11), Some(2), Some(3), Some(14), Some(12)]
    println!("{:?}", deque);

    assert_eq!(deque.pop_rear(), Some(14)); // [Some(11), Some(2), Some(3), None, Some(12)]
    assert_eq!(deque.pop_front(), Some(12)); // [Some(11), Some(2), Some(3), None, None]
    println!("{:?}", deque);
}

#[derive(Debug)]
pub struct Deque<T> {
    size: usize,
    front: usize, // current front index
    next: usize,  // next enqueue index
    data: Vec<Option<T>>,
}

impl<T> Deque<T> {
    pub fn new(cap: usize) -> Self {
        let mut ans = Deque { size: 0, front: 0, next: 0, data: Vec::with_capacity(cap) };

        // T is required to implement Clone for expression data: vec![None; cap];
        for _ in 0..cap {
            ans.data.push(None);
        }

        ans
    }

    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    // enqueue
    pub fn add_rear(&mut self, val: T) -> Result<(), &'static str> {
        if self.capacity() == self.size() {
            return Err("deque is full");
        }

        self.data[self.next] = Some(val);
        self.next = (self.next + 1) % self.capacity();
        self.size += 1;

        Ok(())
    }

    // dequeue
    pub fn pop_front(&mut self) -> Option<T> {
        let ans = self.data[self.front].take()?;

        self.front = (self.front + 1) % self.capacity();
        self.size -= 1;

        Some(ans)
    }

    pub fn add_front(&mut self, val: T) -> Result<(), &'static str> {
        if self.capacity() == self.size() {
            return Err("deque is full");
        }

        self.front = (self.front + self.capacity() - 1) % self.capacity();
        self.data[self.front] = Some(val);
        self.size += 1;

        Ok(())
    }

    pub fn pop_rear(&mut self) -> Option<T> {
        if self.size() == 0 {
            return None;
        }

        self.next = (self.next + self.capacity() - 1) % self.capacity();
        self.size -= 1;

        self.data[self.next].take()
    }
}
