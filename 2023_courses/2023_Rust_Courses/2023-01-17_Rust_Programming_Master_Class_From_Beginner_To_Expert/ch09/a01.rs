fn main() {
    let v1 = Box::new(4.2); // alloc on heap
    assert_eq!(*v1, 4.2);

    let v2 = 42; // on stack
    let v3 = &v2; // on stack
    let v4 = Box::new(v3); // copy from stack to heap
    println!("{:?}", v4);

    assert_eq!(*v4, &42);
}
