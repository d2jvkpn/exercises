#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    #[test]
    fn t01_ref_cell() {
        let v1 = Rc::new(RefCell::new(1));
        let v2 = v1.clone();

        println!("==> v1={:?}, v2={:?}\n", v1, v2);

        *v2.borrow_mut() = 42;
        println!("==> v1={:?}, v2={:?}\n", v1, v2);

        drop(v2); // v2 is a reference of v1, so v1 can't be dropped
        println!("==> v1={:?}\n", v1);
    }

    #[test]
    fn t02_ref_cell() {
        let mut root: Option<Rc<RefCell<usize>>> = None;
        check_root(&mut root);

        println!("==> root: {:?}", root);
    }

    fn check_root(root: &mut Option<Rc<RefCell<usize>>>) {
        if root.is_none() {
            *root = Some(Rc::new(RefCell::new(42)));
        }
    }
}
