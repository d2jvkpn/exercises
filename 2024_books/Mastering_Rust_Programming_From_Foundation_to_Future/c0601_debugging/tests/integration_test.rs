use c0601_debugging::greet;

#[test]
fn t_greet() {
    assert_eq!(greet("Alice"), "Hello, Alice!");
    assert_eq!(greet("Bob"), "Hello, Bob!");
}
