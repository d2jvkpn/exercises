use std::{cell::RefCell, rc::Rc};

struct Trie {
    children: Vec<Child>,
}

struct Node {
    pub value: char,
    pub word: bool,
    pub children: Vec<Child>,
}

type Child = Rc<RefCell<Node>>;

impl Node {
    pub fn new(c: char) -> Self {
        Self { value: c, word: false, children: Vec::with_capacity(1) }
    }

    pub fn into_child(self) -> Child {
        Rc::new(RefCell::new(self))
    }

    pub fn from_chars(chars: &[char]) -> Option<Self> {
        let mut node = match chars.first() {
            None => return None,
            Some(v) => Self::new(*v),
        };

        match chars.split_at(1).1 {
            v if v.is_empty() => node.word = true,
            v => {
                let child = Self::from_chars(v).unwrap().into_child();
                node.children.push(child);
            }
        }

        Some(node)
    }

    fn insert_v0(&mut self, chars: &[char]) {
        let c = match chars.first() {
            None => {
                self.word = true;
                return;
            }
            Some(v) => *v,
        };

        for child in &self.children {
            if child.borrow().value != c {
                continue;
            }

            return child.borrow_mut().insert(&chars[1..]);
        }

        self.children.push(Self::from_chars(chars).unwrap().into_child());
    }

    pub fn insert(&mut self, chars: &[char]) {
        let c = match chars.first() {
            None => return,
            Some(v) => *v,
        };

        for child in &self.children {
            if child.borrow().value != c {
                continue;
            }

            let mut node = child.borrow_mut();
            if chars.len() == 1 {
                node.word = true;
                return;
            }
            return node.insert(&chars[1..]);
        }

        self.children.push(Self::from_chars(chars).unwrap().into_child());
    }

    pub fn search(&self, chars: &[char]) -> bool {
        let c = match chars.first() {
            None => return false,
            Some(v) => *v,
        };

        for child in &self.children {
            let node = child.borrow();
            if node.value != c {
                continue;
            }

            if chars.len() == 1 {
                return true;
            }
            return node.search(&chars[1..]);
        }

        false
    }
}

impl Trie {
    pub fn new() -> Self {
        Self { children: Vec::with_capacity(1) }
    }

    pub fn insert(&mut self, chars: &[char]) {
        let c = match chars.first() {
            None => return,
            Some(v) => *v,
        };

        for child in &self.children {
            let mut node = child.borrow_mut();
            if node.value == c {
                node.insert(&chars[1..]);
                return;
            }
        }

        self.children.push(Node::from_chars(chars).unwrap().into_child());
    }

    pub fn search(&mut self, chars: &[char]) -> bool {
        let c = match chars.first() {
            None => return false,
            Some(v) => *v,
        };

        for child in &self.children {
            let node = child.borrow();
            if node.value == c {
                return node.search(&chars[1..]);
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_trie() {
        let mut trie = Trie::new();

        let chars = &['a', 'p', 'p', 'l', 'e'];
        assert!(!trie.search(chars));
        trie.insert(chars);
        assert!(trie.search(chars));

        let chars = &['a', 'p', 'e'];
        assert!(!trie.search(chars));
        trie.insert(chars);
        assert!(trie.search(chars));
    }
}
