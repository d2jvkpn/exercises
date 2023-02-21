#[derive(Debug)]
struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new(size: usize) -> Self {
        Self { data: Vec::with_capacity(size) }
    }

    pub fn push(&mut self, item: T) -> &mut Self {
        self.data.push(item);
        self
    }

    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    pub fn get(&self, idx: usize) -> Option<&T> {
        self.data.get(idx)
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn peek(&self) -> Option<&T> {
        match self.data.len() {
            0 => None,
            v => self.data.get(v - 1),
        }
    }
}

#[cfg(test)]
mod tests {
    // cargo test -- stack --show-output
    #[test]
    fn t_stack() {
        let mut stack = super::Stack::new(4);
        stack.push(1).push(2);

        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.peek(), Some(&1));
        assert_eq!(stack.len(), 1);
        assert_eq!(stack.get(0), Some(&1));
        assert_eq!(stack.get(1), None);
    }
}
