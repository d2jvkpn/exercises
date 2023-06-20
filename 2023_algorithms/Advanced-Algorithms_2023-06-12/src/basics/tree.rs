use std::{cell::RefCell, cmp::max, fmt::Debug, rc::Rc};

#[derive(Debug)]
pub struct Tree<T> {
    pub root: Child<T>,
    size: usize,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Node<T> {
    pub value: T,
    pub left: Child<T>,
    pub right: Child<T>,
}

pub type Child<T> = Option<Rc<RefCell<Node<T>>>>;

impl<T> From<Node<T>> for Child<T> {
    fn from(node: Node<T>) -> Self {
        Some(Rc::new(RefCell::new(node)))
    }
}

impl<T: PartialEq + PartialOrd + Debug + Clone> Node<T> {
    pub fn new(value: T) -> Self {
        Self { value, left: None, right: None }
    }

    pub fn triangle(value: T, left: T, right: T) -> Self {
        Self { value, left: Self::new(left).into(), right: Self::new(right).into() }
    }

    pub fn push_left(&mut self, node: Node<T>) -> &mut Self {
        if let Some(left) = self.left.take() {
            (*left).borrow_mut().push_left(node);
            self.left = Some(left);
        } else {
            self.left = node.into();
        }

        self
    }

    pub fn push_right(&mut self, node: Node<T>) -> &mut Self {
        /* ?? not workinf
        if let Some(right) = &self.right {
            return right.borrow_mut().push_right(node);
        } else {
            self.left = node.into();
        }
        */

        if let Some(right) = self.right.take() {
            (*right).borrow_mut().push_right(node);
            self.right = Some(right);
        } else {
            self.right = node.into();
        }

        self
    }

    pub fn push(&mut self, left: Node<T>, right: Node<T>) -> &mut Self {
        self.push_left(left);
        self.push_right(right);
        self
    }

    fn count_help(&self, size: &mut usize) {
        if let Some(left) = &self.left {
            left.borrow().count_help(size);
            *size += 1;
        }

        if let Some(right) = &self.right {
            right.borrow().count_help(size);
            *size += 1;
        }
    }

    pub fn count(&self) -> usize {
        let mut result = 0;

        if let Some(left) = &self.left {
            left.borrow().count_help(&mut result);
            result += 1;
        }

        if let Some(right) = &self.right {
            right.borrow().count_help(&mut result);
            result += 1;
        }

        result + 1
    }

    pub fn levels(&self) -> usize {
        // println!("~~~ {:?}, {}, {}", self.value, self.left.is_some(), self.right.is_some());

        let left_levels = match &self.left {
            Some(v) => v.borrow().levels(),
            None => 0,
        };

        let right_levels = match &self.right {
            Some(v) => v.borrow().levels(),
            None => 0,
        };

        max(left_levels, right_levels) + 1
    }
}

impl<T: PartialEq + PartialOrd + Debug + Clone> Tree<T> {
    pub fn new() -> Self {
        Self { root: None, size: 0 }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn levels(&self) -> usize {
        match &self.root {
            Some(root) => root.borrow().levels(),
            None => 0,
        }
    }

    pub fn push_left(&mut self, node: Node<T>) {
        let count = node.count();

        match &self.root {
            Some(root) => _ = root.borrow_mut().push_left(node),
            None => self.root = node.into(),
        }

        self.size += count;
    }

    pub fn push_right(&mut self, node: Node<T>) {
        let count = node.count();

        match &self.root {
            Some(root) => _ = root.borrow_mut().push_right(node),
            None => self.root = node.into(),
        }

        self.size += count;
    }

    pub fn inorder(&self) {}
}

// https://www.jianshu.com/p/7a62dcc96304
// left..., parent, right...
pub fn inorder_recur_a<T: Debug + PartialEq + Clone>(item: &Child<T>) -> Vec<T> {
    let mut result = Vec::new();

    let node = if let Some(v) = item { v } else { return result };

    result.extend(inorder_recur_a(&node.borrow().left));
    result.push(node.borrow().value.clone());
    result.extend(inorder_recur_a(&node.borrow().right));

    result
}

pub fn inorder_recur_b<T: Debug + PartialEq + Clone>(item: &Child<T>) -> Vec<T> {
    fn traversal<T: Debug + PartialEq + Clone>(item: &Child<T>, ans: &mut Vec<T>) {
        let node = if let Some(v) = item { v } else { return };
        traversal(&node.borrow().left, ans);
        ans.push(node.borrow().value.clone());
        traversal(&node.borrow().right, ans);
    }

    let mut ans = vec![];
    traversal(&item, &mut ans);
    ans
}

pub fn inorder_stack<T: Debug + PartialEq + Clone>(item: &Child<T>) -> Vec<T> {
    let mut stack: Vec<Rc<RefCell<Node<T>>>> = vec![];
    let mut ans: Vec<T> = vec![];
    let mut node = item.clone();

    loop {
        if let Some(v) = node {
            stack.push(v.clone());
            node = v.borrow().left.clone();
            continue;
        }

        if let Some(v) = stack.pop() {
            ans.push(v.borrow().value.clone());
            node = v.borrow().right.clone(); // !!
            continue;
        }

        break;
    }

    ans
}

// parent, left..., right...
pub fn preorder_recur<T: Debug + PartialEq + Clone>(item: &Child<T>) -> Vec<T> {
    let mut result = Vec::new();
    let node = if let Some(v) = item { v } else { return result };

    result.push(node.borrow().value.clone());
    result.extend(preorder_recur(&node.borrow().left));
    result.extend(preorder_recur(&node.borrow().right));

    result
}

pub fn preorder_stack<T: Debug + PartialEq + Clone>(item: &Child<T>) -> Vec<T> {
    let mut stack: Vec<Rc<RefCell<Node<T>>>> = vec![];
    let mut ans: Vec<T> = vec![];
    let mut node = item.clone();

    loop {
        if let Some(v) = node {
            ans.push(v.borrow().value.clone());
            if let Some(right) = v.borrow().right.clone() {
                stack.push(right);
            }
            node = v.borrow().left.clone();
            continue;
        }

        if let Some(v) = stack.pop() {
            node = Some(v.clone()); // !!
            continue;
        }

        break;
    }

    ans
}

// left..., right..., parent
pub fn postorder_recur<T: Debug + PartialEq + Clone>(item: &Child<T>) -> Vec<T> {
    let mut result = Vec::new();
    let node = if let Some(v) = item { v } else { return result };

    result.extend(postorder_recur(&node.borrow().left));
    result.extend(postorder_recur(&node.borrow().right));
    result.push(node.borrow().value.clone());

    result
}

pub fn postorder_stack<T: Debug + PartialEq + Clone>(item: &Child<T>) -> Vec<T> {
	todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_tree() {
        let mut tree = Tree::new();

        let mut n1 = Node::new(1);
        // let mut n2 = Node::new(2);
        // let mut n3 = Node::new(3);
        // n2.push(Node::new(4), Node::new(5));
        // n3.push(Node::new(6), Node::new(7));

        n1.push(Node::triangle(2, 4, 5), Node::triangle(3, 6, 7));
        assert_eq!(n1.count(), 7);
        tree.push_left(n1);
        dbg!(&tree);

        assert_eq!(tree.size(), 7);
        assert_eq!(tree.levels(), 3);

        assert_eq!(inorder_recur_a(&tree.root), vec![4, 2, 5, 1, 6, 3, 7]);
        assert_eq!(inorder_recur_b(&tree.root), vec![4, 2, 5, 1, 6, 3, 7]);
        assert_eq!(inorder_stack(&tree.root), vec![4, 2, 5, 1, 6, 3, 7]);

        assert_eq!(preorder_recur(&tree.root), vec![1, 2, 4, 5, 3, 6, 7]);
        assert_eq!(preorder_stack(&tree.root), vec![1, 2, 4, 5, 3, 6, 7]);

        assert_eq!(postorder_recur(&tree.root), vec![4, 5, 2, 6, 7, 3, 1]);
    }
}
