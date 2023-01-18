fn main() {
    let a: &str = "str";
    let b: String = String::from("String");
    let c: &String = &b;

    echo(a);
    echo(c);
    echo(b);

    let result = multiple(4, 2);
    println!("result: ({}, {})", result.0, result.1);

    println!(">>>");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("failed to read input");
    println!("user input: {}", input.trim());
}

fn echo<S: AsRef<str>>(s: S) {
    // call as_ref() to get a &str
    // let str_ref = stringlike.as_ref();

    println!("got: {:?}", s.as_ref());
}

fn multiple(num1: i32, num2: i32) -> (i32, i32) {
    (num1 + num2, num1 * num2)
}
