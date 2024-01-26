fn main() {
    // println!("Hello, wrold!");

    // curring and partial application
    let add_five = add(5);
    println!("==> Curring and Partial Application: add_five(10) = {}.", add_five(10));

    // monads
    let result = divide(10.0, 2.0).and_then(|x| divide(x, 5.0)).and_then(|x| divide(x, 2.0));
    println!("==> Monads: result={result:?}");

    // apply
    println!("==> apply(addition, 4, 2): {}", apply(addition, 4, 2));
    println!("==> apply(multiply, 4, 2): {}", apply(multiply, 4, 2));
    println!("==> apply(xx, 4, 2): {}", apply(xx, 4, 2));

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8];

    println!(
        "==> double even numbers: {:?}",
        numbers.iter().filter(|&x| x % 2 == 0).map(|&x| x * 2).collect::<Vec<_>>(),
    );
}

fn add(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

fn divide(a: f64, b: f64) -> Result<f64, &'static str> {
    if b == 0.0 {
        return Err("divide by zero");
    }

    Ok(a / b)
}

fn apply(operation: fn(i32, i32) -> i32, a: i32, b: i32) -> i32 {
    operation(a, b)
}

fn addition(a: i32, b: i32) -> i32 {
    a + b
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn xx(a: i32, b: i32) -> i32 {
    a * 10 + b
}
