fn main() {
    // println!("Hello, wrold!");

    let greater_than = |x: &i32| *x > 10;
    let less_than = |x: &i32| *x < 20;

    println!("~~~ are_both_true: {}", are_both_true(greater_than, less_than, &15));
}

fn are_both_true<T, U, V>(f1: T, f2: U, item: &V) -> bool
where
    T: Fn(&V) -> bool,
    U: Fn(&V) -> bool,
{
    f1(item) && f2(item)
}
