use c0601::greet;

#[test]
fn t_greet() {
    assert_eq!(greet("Alice"), "Hello, Alice!");
    assert_eq!(greet("Bob"), "Hello, Bob!");
}
