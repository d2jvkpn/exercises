#[derive(PartialEq, Debug)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

#[derive(PartialEq, Debug)]
struct LinkedList<T> {
    header: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Self { value, next: None }
    }
}

impl<T: PartialEq> LinkedList<T> {
    fn new() -> Self {
        Self { header: None }
    }

    fn push_front(&mut self, value: T) -> &mut Self {
        let mut value = Node::new(value);
        value.next = self.header.take();
        self.header = Some(Box::new(value));
        self
    }

    fn pop_front(&mut self) -> Option<Node<T>> {
        let mut value: Node<T> = match self.header.take() {
            Some(v) => *v,
            None => return None,
        };

        self.header = value.next.take();
        Some(value)
    }

    pub fn reverse(&mut self) {
        let mut prev = None;
        let mut curr = self.header.take();

        while let Some(mut v) = curr.take() {
            let next = v.next.take();
            v.next = prev.take();
            prev = Some(v);
            curr = next;
        }

        self.header = prev.take();
    }

    fn find(&self, item: T) -> Option<usize> {
        let mut ans: usize = 0;

        let mut curr: &Node<T> = match &(self.header) {
            None => return None,
            Some(v) => {
                if v.value == item {
                    return Some(ans);
                }
                v
            }
        };

        while let Some(v) = &(curr.next) {
            ans += 1;
            if v.value == item {
                return Some(ans);
            }
            curr = v;
        }

        None
    }

    fn delete(&mut self, item: T) -> Option<usize> {
        let mut ans = 0;

        //
        let mut node: Box<Node<T>> = match self.header.take() {
            None => return None,
            Some(v) => v,
        };
        if node.value == item {
            self.header = node.next.take();
            return Some(ans);
        }
        self.header = Some(node); // return back

        //
        let mut curr: &mut Node<T> = self.header.as_mut().unwrap();

        while let Some(mut v) = curr.next.take() {
            ans += 1;
            if v.value == item {
                curr.next = v.next.take();
                return Some(ans);
            }
            curr.next = Some(v); // return back
            curr = curr.next.as_mut().unwrap();
        }

        None
    }

    fn len(&self) -> usize {
        let mut value: &Node<T> = match &(self.header) {
            None => return 0,
            Some(v) => v,
        };

        let mut size: usize = 1;

        while let Some(ref v) = &(value.next) {
            size += 1;
            value = v;
        }
        size
    }

    pub fn is_palindrome(head: Option<Box<Node<T>>>) -> bool {
        let mut stack = Vec::new();

        let mut curr = match head {
            None => return true,
            Some(v) => v,
        };

        loop {
            stack.push(curr.value);

            curr = match curr.next {
                None => break,
                Some(v) => v,
            };
        }

        for i in 0..stack.len() / 2 {
            if stack[i] != stack[stack.len() - i - 1] {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    // cargo test -- --show-output t_sll
    #[test]
    fn t_sll() {
        let mut list = super::LinkedList::new();
        list.push_front(3).push_front(2).push_front(1);
        dbg!(&list);

        assert_eq!(list.len(), 3);

        assert_eq!(list.pop_front(), Some(super::Node::new(1)));
        assert_eq!(list.len(), 2);

        assert_eq!(list.find(1), None);
        assert_eq!(list.find(3), Some(1));

        assert_eq!(list.delete(1), None);
        assert_eq!(list.delete(3), Some(1));
        dbg!(&list);
    }
}
