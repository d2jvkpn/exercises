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

pub fn inorder_traversal1(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
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

// https://www.jianshu.com/p/7a62dcc96304
pub fn inorder_traversal2(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    fn inorder_dfs(node: &Option<Rc<RefCell<TreeNode>>>, ans: &mut Vec<i32>) {
        match node {
            None => {}
            Some(x) => {
                inorder_dfs(&x.borrow().left, ans);
                ans.push(x.borrow().val);
                inorder_dfs(&x.borrow().right, ans);
            }
        }
    }

    let mut ans = vec![];
    inorder_dfs(&root, &mut ans);
    ans
}

pub fn inorder_traversal3(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    type Link = Option<Rc<RefCell<TreeNode>>>;

    // let mut stack = Vec::new();
    // let mut curr: Option<Rc<RefCell<TreeNode>>> = root;

    let mut stack: Vec<Link> = Vec::new();
    let mut result = Vec::new();
    let mut curr: Link = root;

    loop {
        if curr.is_some() {
            let node = curr.unwrap();
            curr = node.clone().borrow_mut().left.take();
            stack.push(Some(node.clone()));
        } else if !stack.is_empty() {
            let node = stack.pop().flatten().unwrap();
            // let node = stack.pop().unwrap().unwrap();
            result.push(node.clone().borrow().val);
            curr = node.clone().borrow_mut().right.take();
        } else {
            break;
        }
    }

    // stack.iter().map(|v| v.as_ref().unwrap().borrow().val).collect()
    // stack.into_iter().map(|v| v.unwrap().borrow().val).collect()
    result
}

pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    fn traverse(node: &TreeNode, mut depth: usize, ans: &mut Vec<Vec<i32>>) {
        if ans.len() < depth {
            (0..depth - ans.len()).for_each(|_| ans.push(vec![]));
        }
        ans[depth - 1].push(node.val);

        depth += 1;

        if node.left.is_some() {
            traverse(&node.left.as_ref().unwrap().borrow(), depth, ans);
        }

        if node.right.is_some() {
            traverse(&node.right.as_ref().unwrap().borrow(), depth, ans);
        }
    }

    let mut ans = if let Some(node) = root {
        let mut ans = vec![];
        traverse(&node.borrow(), 1, &mut ans);
        ans
    } else {
        vec![]
    };

    (1..ans.len()).step_by(2).for_each(|idx| ans[idx].reverse());

    ans
}
