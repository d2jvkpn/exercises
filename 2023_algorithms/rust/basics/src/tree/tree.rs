use super::node::{Child, Node};
use crate::structs::queue_linked_list::Queue;
use std::{
    cell::RefCell,
    fmt::Debug,
    io::{stdin, stdout, Write},
    rc::Rc,
};

#[derive(Debug)]
pub struct Tree<T> {
    pub root: Child<T>,
    size: usize,
}

impl<T: PartialEq + PartialOrd + Debug + Clone> Default for Tree<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: PartialEq + PartialOrd + Debug + Clone> Tree<T> {
    pub fn new() -> Self {
        Self { root: None, size: 0 }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn height(&self) -> usize {
        match &self.root {
            Some(root) => root.borrow().height(),
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
}

fn build_string_tree() -> Tree<String> {
    let mut queue: Queue<Rc<RefCell<Node<String>>>> = Queue::new();
    let mut input = String::new();
    let mut tree = Tree::new();

    print!("==> Enter root(\"\" -> None, \".\" -> Done): ");

    let _ = stdout().flush();
    // stdin().read_line(&mut input).expect("exit");
    stdin().read_line(&mut input).unwrap();

    // !!! not let root = Node::new(input.trim().to_string());
    let root = Rc::new(RefCell::new(Node::new(input.trim().to_string())));
    tree.root = Some(root.clone());
    queue.push(root);

    while let Some(item) = queue.pop() {
        let node = &item.borrow().item;

        //
        input.clear();
        print!("==> Enter: {:?}.left: ", node.borrow().data);

        let _ = stdout().flush();
        stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string();

        match input.as_str() {
            "" => { /* keep None */ }
            "." => break,
            _ => {
                let left: Rc<RefCell<Node<String>>> = Node::new(input.clone()).into();
                node.borrow_mut().left = Some(left.clone());
                queue.push(left);
            }
        }

        //
        input.clear();
        print!("==> Enter: {:?}.right: ", node.borrow().data);

        let _ = stdout().flush();
        stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string();

        match input.as_str() {
            "" => { /* keep None */ }
            "." => break,
            _ => {
                let right: Rc<RefCell<Node<String>>> = Node::new(input.clone()).into();
                node.borrow_mut().right = Some(right.clone());
                queue.push(right);
            }
        }
    }

    tree
}

#[cfg(test)]
mod tests {
    use super::super::traversal::*;
    use super::*;

    #[test]
    fn t1_tree() {
        let mut tree = Tree::new();

        let mut n1 = Node::new(1);
        // let mut n2 = Node::new(2);
        // let mut n3 = Node::new(3);
        // n2.push(Node::new(4), Node::new(5));
        // n3.push(Node::new(6), Node::new(7));

        n1.set_children(Node::triangle(2, 4, 5), Node::triangle(3, 6, 7));
        assert_eq!(n1.count(), 7);

        tree.push_left(n1);
        dbg!(&tree);
        /*
             1
          2     3
        4  5  6   7
        */

        assert_eq!(tree.size(), 7);
        assert_eq!(tree.height(), 3);

        // depth first search
        let expected = vec![4, 2, 5, 1, 6, 3, 7];
        assert_eq!(inorder_recur_a(&tree.root), expected);
        assert_eq!(inorder_recur_b(&tree.root), expected);
        assert_eq!(inorder_stack(&tree.root), expected);

        let expected = vec![1, 2, 4, 5, 3, 6, 7];
        assert_eq!(preorder_recur(&tree.root), expected);
        assert_eq!(preorder_stack(&tree.root), expected);

        let expected = vec![4, 5, 2, 6, 7, 3, 1];
        assert_eq!(postorder_recur(&tree.root), expected);
        assert_eq!(postorder_stack(&tree.root), expected);

        // breath first search
        let expected = vec![1, 2, 3, 4, 5, 6, 7];
        assert_eq!(breath_first_search(&tree.root), expected);
    }

    #[test]
    fn t2_tree() {
        let tree = build_string_tree();
        println!("==> Tree: {:?}", tree);
        println!("==> breath_first_search: {:?}", breath_first_search(&tree.root));
    }
}
