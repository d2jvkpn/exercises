#![allow(dead_code)]

fn main() {
    // println!("Hello, wrold!");

    let mut queue = Queue::new(3);
    assert!(queue.enqueue(1).is_ok());
    assert!(queue.enqueue(2).is_ok());
    println!("{:?}", queue);

    assert!(queue.enqueue(3).is_ok());

    assert!(queue.enqueue(4).is_err());

    println!("{:?}", queue);

    assert_eq!(queue.dequeue(), Some(1));
    assert_eq!(queue.dequeue(), Some(2));
    assert_eq!(queue.dequeue(), Some(3));

    assert!(queue.is_empty());
    println!("{:?}", queue);
}

#[derive(Debug)]
struct Queue<T> {
    size: usize,
    index: usize,
    data: Vec<Option<T>>,
}

impl<T> Queue<T> {
    pub fn new(cap: usize) -> Self {
        Self { size: 0, index: 0, data: Vec::with_capacity(cap) }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn is_full(&self) -> bool {
        self.size == self.capacity()
    }

    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }

    pub fn enqueue(&mut self, val: T) -> Result<(), &'static str> {
        if self.capacity() == self.size {
            return Err("queue is full");
        }

        if self.data.len() == self.capacity() {
            let next = (self.index + self.size) % self.capacity();
            self.data[next] = Some(val);
        } else {
            self.data.push(Some(val));
        }

        self.size += 1;

        Ok(())
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }

        self.size -= 1;

        let ans = self.data[self.index].take();
        self.index = (self.index + 1) % self.capacity();
        ans
    }
}
