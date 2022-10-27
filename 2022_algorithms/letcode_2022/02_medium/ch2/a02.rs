use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let mut a1 = vec![1, 2, 3];
    let a2 = vec![4, 5, 6];

    a1.extend(&a2);
    println!("{:?}, {:?}", a1, a2);

    let mut a1 = vec![1, 2, 3];
    let a2 = vec![4, 5, 6];

    a1.extend(a2);
    println!("{:?}", a1);

    demo();
}

fn type_name_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

fn demo() {
    //
    let node = Some(Rc::new(RefCell::new(42_i32))); // Option<Rc<RefCell<i32>>>

    if let Some(ref e1) = node {
        println!("{:?}", type_name_of(&e1)); // &Rc<Ref<i32>>
        let e2 = e1.borrow();
        println!("{:?}", type_name_of(&e2)); // Ref<i32>

        let e3 = *e2;
        println!("{:?}", type_name_of(&e3)); // i32
        println!("{:?}", e1); // RefCell { value: 42 }
    }

    println!("{:?}", node); // Some(RefCell { value: 42 })

    //
    let node = Rc::new(RefCell::new(24));
    println!("{:?}, {}", node, type_name_of(&node));

    let val = node.clone();
    println!("{:?}, {:?}", node, val);

    println!("{}", *val.borrow() + 3);
    *val.borrow_mut() += 3;
    println!("{:?}", val);

    //
    let node = Rc::new(RefCell::new(24));
    let n1 = node.clone();
    let n2 = node.clone();
    println!("n1={:?}, n2={:?}", n1, n2);
}
