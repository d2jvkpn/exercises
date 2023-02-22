pub fn hello() {
    println!("Hello from crate call_b!");

    crate::xx::hello();
    crate::xx::a1_hello();
    crate::xx::a2::hello();
}
