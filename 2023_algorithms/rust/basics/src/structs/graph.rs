use std::{cell::RefCell, fmt::Debug, rc::Rc};

#[derive(Clone, Debug, PartialEq)]
pub struct Graph<T> {
    pub root: Node<T>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Node<T> {
    pub item: T,
    pub children: Vec<Child<T>>,
}

// pub type Child<T> = Rc<RefCell<Node<T>>>;
#[derive(Clone, Debug, PartialEq)]
pub struct Child<T> {
    pub item: Rc<RefCell<Node<T>>>,
    pub weight: f64,
}

impl<T: PartialEq + PartialOrd + Debug + Clone> Node<T> {
    pub fn new(item: T) -> Self {
        Self { item, children: vec![] }
    }

    pub fn into_child(self, weight: f64) -> Child<T> {
        Child { item: Rc::new(RefCell::new(self)), weight }
    }

    pub fn connect(&mut self, child: Child<T>) -> Rc<RefCell<Node<T>>> {
        let item = &child.item.borrow().item;

        for v in &self.children {
            if &v.item.borrow().item == item {
                return v.item.clone();
            }
        }

        self.children.push(child.clone());

        child.item.clone()
    }

    pub fn find_child(&self, item: T) -> Option<Rc<RefCell<Node<T>>>> {
        Some(self.children.iter().find(|v| &v.item.borrow().item == &item)?.item.clone())
    }
}

impl<T: PartialEq + PartialOrd + Debug + Clone> Graph<T> {
    pub fn new(node: Node<T>) -> Self {
        Self { root: node }
    }
}
