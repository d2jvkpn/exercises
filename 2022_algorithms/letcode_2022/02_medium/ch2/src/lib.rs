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
        Self { val, left: None, right: None }
    }

    pub fn child(val: i32) -> Rc<RefCell<Self>> {
        let node = Self { val, left: None, right: None };
        Rc::new(RefCell::new(node))
    }

    pub fn as_child(self: Self) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(self))
    }

    pub fn subtree(parent: i32, left: i32, right: i32) -> Self {
        let mut node = Self::new(parent);
        node.left = Some(Self::child(left));
        node.right = Some(Self::child(right));
        node
    }

    pub fn partial_tree(parent: i32, is_left: bool, val: i32) -> Self {
        let mut node = Self::new(parent);
        let child = Self::child(val);
        if is_left {
            node.left = Some(child);
        } else {
            node.right = Some(child);
        }
        node
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

    if let Some(node) = root {
        let mut ans = vec![];
        traverse(&node.borrow(), 1, &mut ans);
        (1..ans.len()).step_by(2).for_each(|idx| ans[idx].reverse());
        ans
    } else {
        vec![]
    }
}

pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    fn new_child(val: i32) -> Rc<RefCell<TreeNode>> {
        let node = TreeNode { val, left: None, right: None };
        Rc::new(RefCell::new(node))
    }

    if preorder.is_empty() {
        return None;
    }

    let root = new_child(preorder[0]);
    let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::with_capacity(preorder.len());

    let mut curr = root.clone();
    let mut j = 0;

    for i in 1..preorder.len() {
        if curr.borrow().val != inorder[j] {
            let left = new_child(preorder[i]);
            curr.borrow_mut().left = Some(left.clone());
            stack.push(curr);
            curr = left;
            continue;
        }

        j += 1;
        while let Some(v) = stack.last() {
            if v.borrow().val == inorder[j] {
                curr = v.clone();
                stack.pop();
                j += 1;
            } else {
                break;
            }
        }

        let right = new_child(preorder[i]);
        curr.borrow_mut().right = Some(right.clone());
        curr = right;
    }

    return Some(root);
}

fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
    let (mut curr, mut mark) = (root, 1);
    let mut stack = Vec::new();

    while !stack.is_empty() || curr.is_some() {
        if curr.is_some() {
            let node = curr.unwrap();
            println!("~~~ curr: {:?}", node.borrow().val);
            stack.push(Some(node.clone()));
            curr = node.borrow_mut().left.take();
        } else {
            let node = stack.pop().flatten().unwrap();
            println!("~~~ pop: {:?}", node.borrow().val);

            if k == mark {
                return node.borrow().val;
            } else {
                mark += 1;
            }
            if let Some(v) = &node.borrow().right {
                curr = Some(v.clone());
            };
        }
    }

    return 0;
}

pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    let mut grid = grid;
    let (m, n) = (grid.len(), grid[0].len());
    let mut count: i32 = 0;

    fn clear(grid: &mut Vec<Vec<char>>, i: usize, j: usize) {
        // println!("~~~ {}, {}", i, j);
        let (m, n) = (grid.len(), grid[0].len());
        if i < m && j < n {
            if grid[i][j] == '0' {
                return;
            }
            grid[i][j] = '0';
        }

        if i > 0 {
            clear(grid, i - 1, j);
        }
        if i < m - 1 {
            clear(grid, i + 1, j);
        }

        if j > 0 {
            clear(grid, i, j - 1);
        }
        if j < n - 1 {
            clear(grid, i, j + 1);
        }
    }

    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == '0' {
                continue;
            }
            count += 1;
            clear(&mut grid, i, j);
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    // [3,9,20,15,7], [9,3,15,20,7]
    #[test]
    fn t_build_tree() {
        let ans = build_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7]);
        println!("<<< {:?}", ans);

        let mut tree = TreeNode::partial_tree(3, true, 9);
        let node = TreeNode::subtree(20, 15, 7);
        tree.right = Some(node.as_child());

        assert_eq!(ans, Some(tree.as_child()));
    }

    #[test]
    fn t_kth_smallest() {
        // https://en.wikipedia.org/wiki/File:Binary_search_tree.svg
        let mut n1 = TreeNode::partial_tree(3, true, 1);
        let node = TreeNode::subtree(6, 4, 7);
        n1.right = Some(node.as_child());

        let mut n2 = TreeNode::new(10);
        let node = TreeNode::partial_tree(14, true, 13);
        n2.right = Some(node.as_child());

        let mut root = TreeNode::new(8);
        root.left = Some(n1.as_child());
        root.right = Some(n2.as_child());

        assert_eq!(kth_smallest(Some(root.as_child()), 3), 4);
    }

    #[test]
    fn t_num_islands() {
        let grid = vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0'],
        ];

        assert_eq!(num_islands(grid), 1);
    }
}
