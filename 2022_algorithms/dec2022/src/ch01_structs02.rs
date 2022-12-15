#![allow(dead_code)]
use std::{any::type_name, cell::RefCell, fmt::Debug, rc::Rc};

fn type_name_of<T: ?Sized>(_: &T) -> String {
    format!("{}", type_name::<T>())
}

///
#[derive(PartialEq, Debug)]
struct LinkedNode<T> {
    val: T,
    next: Option<Rc<RefCell<Self>>>,
}

// std::collections::LinkedList;
#[derive(PartialEq, Debug)]
struct LinkedList<T> {
    header: Rc<RefCell<LinkedNode<T>>>,
}

impl<T: PartialEq + Debug> LinkedNode<T> {
    fn new(val: T) -> Self {
        Self { val, next: None }
    }

    fn as_rc(val: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { val, next: None }))
    }
}

impl<T: PartialEq + Debug> LinkedList<T> {
    fn new(val: T) -> Self {
        Self { header: LinkedNode::as_rc(val) }
    }

    fn push(&mut self, val: T) -> &mut Self {
        let mut curr = self.header.clone();

        while let Some(v) = curr.clone().borrow().next.as_ref() {
            curr = v.clone();
        }

        curr.clone().borrow_mut().next = Some(LinkedNode::as_rc(val));
        self
    }

    fn pop(&mut self) -> Option<Rc<RefCell<LinkedNode<T>>>> {
        let mut curr = self.header.clone();

        while let Some(v) = curr.clone().borrow().next.as_ref() {
            if v.borrow().next.is_none() {
                // the next of curr is the last element
                // return curr.borrow_mut().next.take(); // panicked at 'already borrowed: BorrowMutError'
                break;
            }
            curr = v.clone();
        }
        let ans = curr.borrow_mut().next.take();
        ans
    }

    fn pop_front(&mut self) -> Option<Rc<RefCell<LinkedNode<T>>>> {
        let curr = self.header.clone();

        match curr.borrow_mut().next.take() {
            None => return None,
            Some(v) => self.header = v,
        };

        Some(curr)
    }

    fn find(&self, item: LinkedNode<T>) -> Option<usize> {
        let mut curr = self.header.clone();
        let item = Rc::new(RefCell::new(item));
        if curr.borrow().val == item.borrow().val {
            return Some(0);
        }

        let mut ans: usize = 0;

        while let Some(v) = curr.clone().borrow().next.as_ref() {
            ans += 1;
            if v.borrow().val == item.borrow().val {
                return Some(ans);
            }
            curr = v.clone();
        }

        None
    }

    fn len(&self) -> usize {
        let mut curr = self.header.clone();
        let mut size: usize = 1;

        while let Some(v) = curr.clone().borrow().next.as_ref() {
            size += 1;
            curr = v.clone();
        }
        size
    }
}

#[cfg(test)]
mod tests {
    // $ cargo test --lib -- --show-output ch01_structs02::tests::linked_list
    #[test]
    fn linked_list() {
        let mut list = super::LinkedList::new(1);
        list.push(2).push(3);

        assert_eq!(list.find(super::LinkedNode::new(1)), Some(0));
        assert_eq!(list.find(super::LinkedNode::new(3)), Some(2));

        assert_eq!(list.pop(), Some(super::LinkedNode::as_rc(3)));
        assert_eq!(list.len(), 2);
        // dbg!(&list);

        assert_eq!(list.pop_front(), Some(super::LinkedNode::as_rc(1)))
    }
}
