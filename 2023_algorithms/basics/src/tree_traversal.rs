use super::{queue::Queue, tree_node::Child};
use std::fmt::Debug;

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
    let mut stack = vec![]; // Vec<Rc<RefCell<Node<T>>>>
    let mut ans = vec![]; // Vec<T>
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
    let mut stack = vec![]; // Vec<Rc<RefCell<Node<T>>>>
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
    let mut stack = vec![]; // Vec<Rc<RefCell<Node<T>>>>
    let mut visited: Vec<bool> = vec![];
    let mut ans: Vec<T> = vec![];
    let node = item.clone();

    if let Some(v) = node {
        stack.push(v.clone());
        visited.push(false);
    } else {
        return ans;
    }

    while let (Some(node), Some(viz)) = (stack.pop(), visited.pop()) {
        if viz {
            ans.push(node.borrow().value.clone());
        } else {
            stack.push(node.clone());
            visited.push(true);

            if let Some(v) = node.borrow().right.clone() {
                stack.push(v);
                visited.push(false);
            }

            if let Some(v) = node.borrow().left.clone() {
                stack.push(v);
                visited.push(false);
            }
        }
    }

    ans
}

pub fn breath_first_search<T: Debug + PartialEq + Clone>(item: &Child<T>) -> Vec<T> {
    let mut vec = Vec::new();

    let mut queue = match item {
        None => return vec,
        Some(v) => Queue::new_with(v.clone()),
    };

    while let Some(qn) = queue.pop() {
        if let Some(v) = &qn.borrow().value.borrow().left {
            _ = queue.push(v.clone());
        }

        if let Some(v) = &qn.borrow().value.borrow().right {
            _ = queue.push(v.clone());
        }

        vec.push(qn.borrow().value.borrow().value.clone());
    }

    vec
}
