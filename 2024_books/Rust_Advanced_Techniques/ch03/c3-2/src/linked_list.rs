use std::{cell::RefCell, fmt, rc::Rc};

// #[derive(Debug)]
pub struct Node<T> {
	item: Rc<RefCell<T>>,
	next: NextNode<T>,
}

type NextNode<T> = Option<Rc<RefCell<Node<T>>>>;

impl<T> Node<T> {
	pub fn new(item: T) -> Self {
		Self { item: Rc::new(RefCell::new(item)), next: None }
	}

	pub fn push(&mut self, item: T) {
		match &self.next {
			None => self.next = Node::new(item).into(),
			Some(v) => v.borrow_mut().push(item),
		}
	}
}

impl<T: fmt::Debug> fmt::Debug for Node<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
		let mut vec = Vec::new();
		vec.push(self.item.clone());

		let mut next = self.next.clone();
		while let Some(node) = next {
			vec.push(node.borrow().item.clone());
			next = node.borrow().next.clone();
		}

		write!(f, "{:?}", vec)
	}
}

impl<T> From<Node<T>> for NextNode<T> {
	fn from(node: Node<T>) -> Self {
		Some(Rc::new(RefCell::new(node)))
	}
}

#[derive(Debug)]
pub struct LinkedList<T> {
	head: NextNode<T>,
	size: usize,
	cursor: NextNode<T>,
}

impl<T> LinkedList<T> {
	pub fn new(t: T) -> Self {
		let head: NextNode<T> = Node::new(t).into();
		Self { head: head.clone(), size: 1, cursor: head.clone() }
	}

	pub fn push(&mut self, d: T) -> &mut Self {
		match &self.head {
			None => self.head = Node::new(d).into(),
			Some(v) => v.borrow_mut().push(d),
		};

		self.size += 1;
		self
	}

	pub fn size(&self) -> usize {
		self.size
	}

	pub fn reset_cursor(&mut self) {
		self.cursor = self.head.clone();
	}

	pub fn get_head(&self) -> NextNode<T> {
		self.head.clone()
	}
}

impl<T> Iterator for LinkedList<T> {
	type Item = Rc<RefCell<T>>;

	fn next(&mut self) -> Option<Self::Item> {
		let cursor = self.cursor.clone()?;
		let ans = cursor.borrow();
		self.cursor = ans.next.clone();
		Some(ans.item.clone())
	}
}
