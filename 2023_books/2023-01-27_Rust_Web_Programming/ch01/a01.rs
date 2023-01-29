use std::collections::HashMap;

fn main() {
    //
    let s = &"one";
    println!("{}", s);
    print_type_of(&s);

    let mut d = HashMap::new();
    d.insert("hello", 42);
    println!("{:?}", d);

    //
    let mut d = HashMap::from([("hello".to_string(), 42)]);
    d.entry("world".to_string()).or_insert(24);
    println!("{:?}", d);
    assert_eq!(d.get("world").unwrap(), &24);

    d.iter_mut().for_each(|(_, v)| *v += 1);
    println!("{:?}", d);

    for (k, v) in &d {
        println!("~~~ k={k:?}, v={v}");
    }

    for k in d.into_keys() {
        println!("k={:?}", k);
    }
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
