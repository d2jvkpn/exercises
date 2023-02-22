use std::{cell::RefCell, fmt::Debug, rc::Rc};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Node<T> {
    pub value: T,
    pub left: Option<Rc<RefCell<Node<T>>>>,
    pub right: Option<Rc<RefCell<Node<T>>>>,
}

#[derive(Debug)]
struct BinaryTree<T> {
    root: Node<T>,
    size: usize,
}

impl<T: PartialEq + PartialOrd + Debug + Clone> Node<T> {
    fn new(value: T) -> Self {
        Self { value, left: None, right: None }
    }

    fn into_rc(self) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(self))
    }

    fn add(&mut self, value: T) {
        if value <= self.value {
            if let Some(node) = self.left.take() {
                println!("    <== walk left ({:?}, {:?})", self.value, node.borrow().value);
                (*node).borrow_mut().add(value); // !!! not *node.borrow_mut().add(value)
                self.left = Some(node); // must return self.left
            } else {
                println!("    <++ new left {:?}.left = {:?}\n", self.value, value);
                let node = Node::new(value);
                self.left = Some(Rc::new(RefCell::new(node)));
                // println!("{} {:?}", self.value, self.left);
            }
        } else {
            if let Some(node) = self.right.take() {
                println!("    ==> walk right ({:?}, {:?})", self.value, node.borrow().value);
                (*node).borrow_mut().add(value);
                self.right = Some(node); // must return to self.right
            } else {
                println!("    ++> add right {:?}.right = {:?}\n", self.value, value);
                self.right = Some(Node::new(value).into_rc());
            }
        }
    }

    fn find(&self, value: T, steps: &mut Vec<bool>) {
        if self.value == value {
            return;
        }

        if value < self.value {
            if let Some(node) = &self.left {
                steps.push(false);
                node.borrow().find(value, steps);
            } else {
                *steps = vec![];
            }
        } else {
            if let Some(node) = &self.right {
                steps.push(true);
                node.borrow().find(value, steps);
            } else {
                *steps = vec![];
            }
        }
    }

    fn get(&self, steps: &[bool]) -> Option<T> {
        if steps.len() == 0 {
            return Some(self.value.clone());
        }

        if !steps[0] {
            match &self.left {
                Some(n) => n.borrow().get(&steps[1..]),
                None => return None,
            }
        } else {
            match &self.right {
                Some(n) => n.borrow().get(&steps[1..]),
                None => return None,
            }
        }
        // use a for loop instead??
    }
}

impl<T: Clone + Debug + PartialEq + PartialOrd> BinaryTree<T> {
    fn new(value: T) -> Self {
        Self { root: Node::new(value), size: 1 }
    }
    // left.borrow_mut().add(value);
    fn add(&mut self, value: T) -> &mut Self {
        self.root.add(value);
        self.size += 1;
        self
    }

    fn find(&self, value: T) -> Option<Vec<bool>> {
        let mut steps = Vec::with_capacity(10);
        steps.push(false); // root node

        self.root.find(value, &mut steps);

        if steps.len() == 0 {
            None
        } else {
            Some(steps[1..].to_vec())
        }
    }

    fn get(&self, steps: &Vec<bool>) -> Option<T> {
        self.root.get(&steps[0..])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_binary_tree() {
        let mut bt = BinaryTree::new(10);
        println!("{:?}", bt);

        println!("~~~");
        bt.add(5).add(1);

        println!("~~~");
        bt.add(12);

        println!("~~~");
        bt.add(4).add(6).add(8);

        println!("{:?}", bt.root);

        println!("find\t{}\t{:?}", 10, bt.find(10)); // Some([])
        println!("find\t{}\t{:?}", 1, bt.find(1)); // Some([false, false])
        println!("find\t{}\t{:?}", 100, bt.find(100)); // None

        let v1 = bt.find(8).unwrap();
        println!("find\t{}\t{:?}", 8, v1); // Some([false, true, true])
        println!("get\t{:?}\t{:?}", v1, bt.get(&v1)); // Some(8)
    }
}
