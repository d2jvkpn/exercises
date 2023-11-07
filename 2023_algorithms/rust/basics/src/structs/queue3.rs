use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Queue<T> {
    pub data: Vec<Option<T>>,

    pub max_size: usize,
    pub size: usize,
    pub front: usize,
    pub rear: usize,
}

impl<T: Debug + Clone + PartialEq> Queue<T> {
    pub fn new(max_size: usize) -> Self {
        Queue {
            data: vec![None; max_size],

            max_size,
            size: 0,
            front: 0,           // index
            rear: max_size - 1, // index
        }
    }

    pub fn show(&self) {
        println!(
            "data: {:?}, max_size: {}, size: {}, front: {}, rear: {}",
            self.data, self.max_size, self.size, self.front, self.rear,
        )
    }

    pub fn is_empty(&self) -> bool {
        self.size >= self.max_size
    }

    pub fn push(&mut self, item: T) -> Result<(), &'static str> {
        if self.size >= self.max_size {
            return Err("queue is full");
        }

        self.rear = (self.rear + 1) % self.max_size;
        self.size += 1;

        self.data[self.rear % self.max_size] = Some(item);

        Ok(())
    }

    pub fn pop(&mut self) -> Result<T, &'static str> {
        if self.is_empty() {
            return Err("queue is empty");
        }

        let ans = self.data[self.front].take().ok_or("unexpected")?;

        self.size -= 1;
        self.front = (self.front + 1) % self.max_size;

        Ok(ans)
    }

    pub fn front(&self) -> Result<Option<&T>, &'static str> {
        if self.is_empty() {
            return Err("queue is empty");
        }

        let ans = &self.data[self.front];

        Ok(ans.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_queue3() {
        let mut queue: Queue<usize> = Queue::new(7);

        queue.push(42).unwrap();
        queue.push(27).unwrap();
        queue.push(1).unwrap();
        queue.show();

        assert_eq!(queue.pop(), Ok(42));
        assert_eq!(queue.front(), Ok(Some(&27)));
        assert_eq!(queue.pop(), Ok(27));

        queue.show();
    }
}
