fn main() {
    let mut list = List::new();
    list.push(1).push(2).push(4).push(8);
}

type Next<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct List<T> {
    size: usize,
    head: Next<T>,
}

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Next<T>,
}

impl<T> From<Node<T>> for Next<T> {
    fn from(node: Node<T>) -> Next<T> {
        Some(Box::new(node))
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self { size: 0, head: None }
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn push(&mut self, elem: T) -> &mut Self {
        self.head = Node { elem, next: self.head.take() }.into();
        self.size += 1;
        self
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|mut node| {
            self.head = node.next.take();
            self.size -= 1;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }
}
