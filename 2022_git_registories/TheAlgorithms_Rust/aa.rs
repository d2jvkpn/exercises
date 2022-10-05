fn main() {
    println!("Hello, wrold!");

    let a = Some(42_i64);
    println!("{:?}", a);

    echo(a);
}

fn echo(value: Option<i64>) {
    let v = match value {
        Some(v) => v,
        None => return,
    };

    println!("value = {}", v);
}
