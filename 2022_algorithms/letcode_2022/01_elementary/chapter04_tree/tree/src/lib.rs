use std::cell::RefCell;
use std::rc::Rc;

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

pub fn max_depth1(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn max_depth_dfs(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if node.is_none() {
            return 0;
        }

        let curr = &node.as_ref().unwrap().borrow();
        let h1 = max_depth_dfs(&curr.left);
        let h2 = max_depth_dfs(&curr.right);

        return if h1 > h2 { h1 + 1 } else { h2 + 1 };
    }

    max_depth_dfs(&root)
}

pub fn max_depth2(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn max_depth_dfs(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if node.is_none() {
            return 0;
        }

        let curr = &node.as_ref().unwrap().borrow();
        let h1 = max_depth_dfs(&curr.left);
        let h2 = max_depth_dfs(&curr.right);

        return if h1 > h2 { h1 + 1 } else { h2 + 1 };
    }

    max_depth_dfs(&root)
}
