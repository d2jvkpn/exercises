use std::rc::Rc;
// Rc is not thread safe

fn main() {
    let a = Rc::new([1, 2, 3]);
    let b = a.clone();
    assert_eq!(a.as_ptr(), b.as_ptr());
}
