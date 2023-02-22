use std::{cell::RefCell, cmp::PartialEq, fmt::Debug, rc::Rc};

#[derive(Debug)]
pub struct Node<T: Debug + PartialEq> {
    pub value: T,
    pub left: Option<Rc<RefCell<Node<T>>>>,
    pub right: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Debug + PartialEq> Node<T> {
    pub fn new(value: T) -> Self {
        Self { value, left: None, right: None }
    }

    pub fn into_rc(self) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(self))
    }

    pub fn count(&self) -> usize {
        let mut num = 1;

        match &self.left {
            None => {}
            Some(v) => num += v.borrow().count(),
        }

        match &self.right {
            None => {}
            Some(v) => num += v.borrow().count(),
        }

        num
    }
}

#[derive(Debug)]
pub struct Tree<T: Debug + PartialEq> {
    pub header: Rc<RefCell<Node<T>>>,
}

impl<T: Debug + PartialEq> Tree<T> {
    pub fn new(value: T) -> Self {
        Self { header: Node::new(value).into_rc() }
    }
}

// https://www.jianshu.com/p/7a62dcc96304
// left..., node, right...
pub fn inorder_v1<T: Debug + PartialEq + Clone>(item: &Option<Rc<RefCell<Node<T>>>>) -> Vec<T> {
    let mut result = Vec::new();

    let node = if let Some(v) = item { v } else { return result };

    result.extend(inorder_v1(&node.borrow().left));
    result.push(node.borrow().value.clone());
    result.extend(inorder_v1(&node.borrow().right));

    result
}

pub fn inorder_v2<T: Debug + PartialEq + Clone>(item: Option<Rc<RefCell<Node<T>>>>) -> Vec<T> {
    fn traversal<T: Debug + PartialEq + Clone>(
        item: &Option<Rc<RefCell<Node<T>>>>,
        ans: &mut Vec<T>,
    ) {
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
pub fn inorder_v3<T: Debug + PartialEq + Clone>(item: Option<Rc<RefCell<Node<T>>>>) -> Vec<T> {
    let mut ans = Vec::new();
    let mut stack: Vec<Option<Rc<RefCell<Node<T>>>>> = Vec::new();
    let mut curr: Option<Rc<RefCell<Node<T>>>> = item;

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

pub fn postorder<T: Debug + PartialEq + Clone>(item: &Option<Rc<RefCell<Node<T>>>>) -> Vec<T> {
    let mut result = Vec::new();
    let node = if let Some(v) = item { v } else { return result };

    result.extend(postorder(&node.borrow().left));
    result.extend(postorder(&node.borrow().right));
    result.push(node.borrow().value.clone());

    result
}

pub fn preorder<T: Debug + PartialEq + Clone>(item: &Option<Rc<RefCell<Node<T>>>>) -> Vec<T> {
    let mut result = Vec::new();
    let node = if let Some(v) = item { v } else { return result };

    result.push(node.borrow().value.clone());
    result.extend(preorder(&node.borrow().left));
    result.extend(preorder(&node.borrow().right));

    result
}
