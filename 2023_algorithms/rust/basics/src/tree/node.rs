use crate::structs::queue_linked_list::Queue;
use std::{
    cell::RefCell,
    cmp::max,
    fmt,
    io::{stdin, stdout, Write},
    mem,
    ops::Not,
    rc::Rc,
};

pub type Child<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Node<T> {
    pub data: T,
    pub left: Child<T>,
    pub right: Child<T>,
}

#[derive(Copy, Clone, Debug)]
pub enum Side {
    Left,
    Right,
}

impl Not for Side {
    type Output = Side;

    fn not(self) -> Self::Output {
        match self {
            Side::Left => Side::Right,
            Side::Right => Side::Left,
        }
    }
}

impl<T> From<Node<T>> for Rc<RefCell<Node<T>>> {
    fn from(node: Node<T>) -> Self {
        Rc::new(RefCell::new(node))
    }
}

impl<T> From<Node<T>> for Child<T> {
    fn from(node: Node<T>) -> Self {
        Some(Rc::new(RefCell::new(node)))
    }
}

/*
impl<T> Drop for Node<T> {
    fn drop(&mut self) {
        dbg!("==> drop Node.");
    }
}
*/

impl<T: fmt::Display> fmt::Display for Node<T> {
    fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
        let left = match &self.left {
            Some(v) => format!("{}", v.borrow().data),
            None => "".to_string(),
        };

        let right = match &self.right {
            Some(v) => format!("{}", v.borrow().data),
            None => "".to_string(),
        };

        write!(w, "{}({}, {})", self.data, left, right)
    }
}

impl<T: PartialEq + PartialOrd + fmt::Debug + Clone> Node<T> {
    pub fn new(data: T) -> Self {
        Self { data, left: None, right: None }
    }

    pub fn triangle(data: T, left: T, right: T) -> Self {
        Self { data, left: Self::new(left).into(), right: Self::new(right).into() }
    }

    pub fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }

    pub fn set_children(&mut self, left: Node<T>, right: Node<T>) {
        self.left = left.into();
        self.right = right.into();
    }

    fn count_recur(&self, ans: &mut usize) {
        if let Some(left) = &self.left {
            left.borrow().count_recur(ans);
            *ans += 1;
        }

        if let Some(right) = &self.right {
            right.borrow().count_recur(ans);
            *ans += 1;
        }
    }

    pub fn count(&self) -> usize {
        let mut ans = 0;

        if let Some(left) = &self.left {
            left.borrow().count_recur(&mut ans);
            ans += 1;
        }

        if let Some(right) = &self.right {
            right.borrow().count_recur(&mut ans);
            ans += 1;
        }

        ans + 1
    }

    pub fn height(&self) -> usize {
        let left = match &self.left {
            Some(v) => v.borrow().height(),
            None => 0,
        };

        let right = match &self.right {
            Some(v) => v.borrow().height(),
            None => 0,
        };

        max(left, right) + 1
    }

    pub fn balance_factor(&self) -> isize {
        let (mut left, mut right) = (0, 0);

        if let Some(v) = &self.left {
            left = v.borrow().height() as isize;
        }

        if let Some(v) = &self.right {
            right = v.borrow().height() as isize;
        }

        return left - right;
    }

    pub fn is_none(&self, side: Side) -> bool {
        self.child(side).is_none()
    }

    pub fn child(&self, side: Side) -> &Child<T> {
        match side {
            Side::Left => &self.left,
            Side::Right => &self.right,
        }
    }

    pub fn child_mut(&mut self, side: Side) -> &mut Child<T> {
        match side {
            Side::Left => &mut self.left,
            Side::Right => &mut self.right,
        }
    }

    pub fn set_child(&mut self, side: Side, child: Child<T>) {
        match side {
            Side::Left => self.left = child,
            Side::Right => self.right = child,
        }
    }

    pub fn take_child(&mut self, side: Side) -> Child<T> {
        match side {
            Side::Left => self.left.take(),
            Side::Right => self.right.take(),
        }
    }

    pub fn rotate(&mut self, side: Side) {
        let subtree = match self.take_child(!side).take() {
            Some(v) => v,
            None => return,
        };

        self.set_child(!side, subtree.borrow_mut().take_child(side));
        mem::swap(self, &mut subtree.borrow_mut());
        self.set_child(side, Some(subtree));
    }

    pub fn levels_from_vec(vec: Vec<T>) -> Option<Self> {
        // dbg!(&vec);
        if vec.is_empty() {
            return None;
        }

        let mut queue: Queue<Rc<RefCell<Node<T>>>> = Queue::new();

        let tree = Rc::new(RefCell::new(Node::new(vec[0].clone())));
        queue.push(tree.clone().into());

        for i in (1..vec.len()).step_by(2) {
            let parent = match queue.pop() {
                Some(v) => v,
                None => break,
            };

            let left: Rc<RefCell<Node<T>>> = Node::new(vec[i].clone()).into();
            parent.borrow_mut().left = Some(left.clone());
            queue.push(left);

            if i + 1 >= vec.len() {
                continue;
            }

            let right: Rc<RefCell<Node<T>>> = Node::new(vec[i + 1].clone()).into();
            parent.borrow_mut().right = Some(right.clone());
            queue.push(right);
        }

        let ans = tree.borrow().clone();
        Some(ans)
    }

    fn levels_print(&self) {
        println!("==> Levels Print:");
        let mut queue: Queue<Rc<RefCell<Node<T>>>> = Queue::new();

        queue.push(self.clone().into());

        while let Some(node) = queue.pop() {
            let left = match &node.borrow().left {
                Some(v) => {
                    let v = v.clone();
                    let ans = format!("{:?}", v.borrow().data);
                    queue.push(v);
                    ans
                }
                None => "".to_string(),
            };

            let right = match &node.borrow().right {
                Some(v) => {
                    let v = v.clone();
                    let ans = format!("{:?}", v.borrow().data);
                    queue.push(v);
                    ans
                }
                None => "".to_string(),
            };

            print!("{:?}({}, {})-> ", node.borrow().data, left, right);
        }

        // dbg!(&queue);
        println!("");
    }

    // return: (None, None), (None, Some), (Some, Some)
    fn take_most(item: &Child<T>, side: Side) -> (Child<T>, Child<T>) {
        // parent, target
        let node = if let Some(v) = item {
            v
        } else {
            return (None, None); // not found
        };

        let binding = node.borrow();
        let child = match binding.child(side) {
            None => return (None, Some(node.clone())), // has no parent
            Some(v) => v,
        };

        if child.borrow().child(side).is_none() {
            node.borrow_mut().set_child(side, child.borrow_mut().take_child(!side));
            return (Some(node.clone()), Some(child.clone()));
        }

        Self::take_most(&Some(child.clone()), side)
    }

    // TODO: move to bst tree
    /*
    pub fn bst_push(&mut self, value: T) -> bool {
        let mut ans = true;

        if value == self.data {
            ans = false;
        } else if value < self.data {
            if let Some(v) = self.left.take() {
                ans = (*v).borrow_mut().bst_push(value); // !!! not *v.borrow_mut().push(data)
                self.left = Some(v); // must return self.left
            } else {
                self.left = Node::new(value.clone()).into();
            }
        } else {
            if let Some(v) = self.right.take() {
                ans = (*v).borrow_mut().bst_push(value);
                self.right = Some(v);
            } else {
                self.right = Node::new(value.clone()).into();
            }
        }

        ans
    }
    */

    fn bst_push_recur(child: &Child<T>, value: T) -> bool {
        let node = match child {
            None => return false,
            Some(v) => v,
        };

        let side = if node.borrow().data == value {
            return false;
        } else if node.borrow().data < value {
            Side::Left
        } else {
            Side::Right
        };

        if node.borrow().child(side).is_none() {
            node.borrow_mut().set_child(side, Node::new(value).into());
            true
        } else {
            Self::bst_push_recur(node.borrow().child(side), value)
        }
    }

    pub fn bst_push(&self, value: T) -> bool {
        Self::bst_push_recur(&(self.clone()).into(), value)
    }
}

enum EnterString {
    None,
    Quit,
    Value(String),
}

fn enter_node() -> EnterString {
    if stdout().flush().is_err() {
        return EnterString::None;
    }

    let mut input = String::new();

    if stdin().read_line(&mut input).is_err() {
        return EnterString::None;
    };

    input = input.trim().to_string();

    match input.as_str() {
        "" => EnterString::None,
        "." => EnterString::Quit,
        _ => EnterString::Value(input),
    }
}

pub fn levels_build() -> Child<String> {
    let mut queue: Queue<Rc<RefCell<Node<String>>>> = Queue::new();

    print!("==> Enter root({:?} -> None, {:?} -> Done): ", "", ".");

    let tree: Rc<RefCell<Node<String>>> = match enter_node() {
        EnterString::None | EnterString::Quit => return None,
        EnterString::Value(v) => Node::new(v).into(),
    };

    queue.push(tree.clone());

    while let Some(node) = queue.pop() {
        print!("==> Enter: {:?}.left: ", node.borrow().data);

        match enter_node() {
            EnterString::None => {}
            EnterString::Quit => break,
            EnterString::Value(v) => {
                let left: Rc<RefCell<Node<String>>> = Node::new(v).into();
                node.borrow_mut().left = Some(left.clone());
                queue.push(left);
            }
        };

        print!("==> Enter: {:?}.right: ", node.borrow().data);

        match enter_node() {
            EnterString::None => {}
            EnterString::Quit => break,
            EnterString::Value(v) => {
                let right: Rc<RefCell<Node<String>>> = Node::new(v).into();
                node.borrow_mut().right = Some(right.clone());
                queue.push(right);
            }
        };
    }

    Some(tree)
}

#[cfg(test)]
mod tests {
    use super::super::traversal::*;
    use super::*;

    #[test]
    fn t1_node() {
        let mut n1 = Node::new(1);
        // let mut n2 = Node::new(2);
        // let mut n3 = Node::new(3);
        // n2.push(Node::new(4), Node::new(5));
        // n3.push(Node::new(6), Node::new(7));

        n1.set_children(Node::triangle(2, 4, 5), Node::triangle(3, 6, 7));
        /*
             1
          2     3
        4  5  6   7
        */

        assert_eq!(n1.count(), 7);
        assert_eq!(n1.height(), 3);

        let root = Some(Rc::new(RefCell::new(n1)));

        // depth first search
        let expected = vec![4, 2, 5, 1, 6, 3, 7];
        assert_eq!(inorder_recur_a(&root), expected);
        assert_eq!(inorder_recur_b(&root), expected);
        assert_eq!(inorder_stack(&root), expected);

        let expected = vec![1, 2, 4, 5, 3, 6, 7];
        assert_eq!(preorder_recur(&root), expected);
        assert_eq!(preorder_stack(&root), expected);

        let expected = vec![4, 5, 2, 6, 7, 3, 1];
        assert_eq!(postorder_recur(&root), expected);
        assert_eq!(postorder_stack(&root), expected);

        // breath first search
        let expected = vec![1, 2, 3, 4, 5, 6, 7];
        assert_eq!(breath_first_search(&root), expected);
    }

    #[test]
    fn t2_node() {
        let mut n1 = Node::new(1);

        n1.set_children(Node::triangle(2, 4, 5), Node::triangle(3, 6, 7));
        assert_eq!(n1.count(), 7);

        n1.levels_print();
        n1.rotate(Side::Left);
        n1.levels_print();
    }

    #[test]
    fn t3_node() {
        /*
        let tree = levels_build().unwrap();
        tree.borrow_mut().rotate(Side::Left);
        tree.borrow().levels_print();

        let tree = levels_build().unwrap();
        tree.borrow_mut().rotate(Side::Right);
        tree.borrow().levels_print();
        */

        let mut tree = Node::levels_from_vec((1..=7).collect()).unwrap();
        tree.rotate(Side::Right);
        tree.levels_print();
    }
}
