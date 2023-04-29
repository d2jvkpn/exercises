use crate::node::Child;
use std::{cmp::PartialEq, fmt::Debug};

// https://www.jianshu.com/p/7a62dcc96304
// left..., parent, right...
pub fn inorder_v1<T: Debug + PartialEq + Clone>(item: &Child<T>) -> Vec<T> {
    let mut result = Vec::new();

    let node = if let Some(v) = item { v } else { return result };

    result.extend(inorder_v1(&node.borrow().left));
    result.push(node.borrow().value.clone());
    result.extend(inorder_v1(&node.borrow().right));

    result
}

// left..., parent, right...
pub fn inorder_v2<T: Debug + PartialEq + Clone>(item: &Child<T>) -> Vec<T> {
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

// the tree will be destoryed
pub fn inorder_v3<T: Debug + PartialEq + Clone>(item: Child<T>) -> Vec<T> {
    let mut ans = Vec::new();
    let mut stack: Vec<Child<T>> = Vec::new();
    let mut curr: Child<T> = item;

    loop {
        if curr.is_some() {
            let node = curr.unwrap();
            curr = node.clone().borrow_mut().left.take();
            stack.push(Some(node.clone()));
        } else if !stack.is_empty() {
            let node = stack.pop().flatten().unwrap();
            ans.push(node.clone().borrow().value.clone());
            curr = node.clone().borrow_mut().right.take();
        } else {
            break;
        }
    }

    ans
}

// parent, left..., right...
pub fn preorder_v1<T: Debug + PartialEq + Clone>(item: &Child<T>) -> Vec<T> {
    let mut result = Vec::new();
    let node = if let Some(v) = item { v } else { return result };

    result.push(node.borrow().value.clone());
    result.extend(preorder_v1(&node.borrow().left));
    result.extend(preorder_v1(&node.borrow().right));

    result
}

// left..., right..., parent
pub fn postorder_v1<T: Debug + PartialEq + Clone>(item: &Child<T>) -> Vec<T> {
    let mut result = Vec::new();
    let node = if let Some(v) = item { v } else { return result };

    result.extend(postorder_v1(&node.borrow().left));
    result.extend(postorder_v1(&node.borrow().right));
    result.push(node.borrow().value.clone());

    result
}
