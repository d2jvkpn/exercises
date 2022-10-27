use std::cell::RefCell;
use std::rc::Rc;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[allow(dead_code)]
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode { val, left: None, right: None }
    }
}

pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    fn traversal(node: &TreeNode) -> Vec<i32> {
        let mut result = Vec::new();

        if node.left.is_some() {
            result.extend(traversal(&node.left.as_ref().unwrap().borrow()));
        }

        result.push(node.val);

        if node.right.is_some() {
            result.extend(traversal(&node.right.as_ref().unwrap().borrow()));
        }

        result
    }

    if root.is_some() {
        traversal(&root.as_ref().unwrap().borrow())
    } else {
        vec![]
    }
}

pub fn inorder_traversal2(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    todo!()
}
