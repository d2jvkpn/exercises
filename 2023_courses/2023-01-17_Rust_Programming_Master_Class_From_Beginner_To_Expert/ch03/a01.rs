fn main() {
    let a = (1, 2, 3);
    let b = a;
    println!("a={:?}, b={:?}", a, b);

    let a = (1, 2, "3".to_string());
    // let b = a; // moved
    let b = a.clone(); // ok
    println!("a={:?}, b={:?}", a, b);

    let a = (1, 2, "3".to_string());
    let b = &a; // borrowing and referencing
    println!("a={:?}, b={:?}", a, b);

    let mut vec1: Vec<i32> = vec![1, 2, 3];
    {
        vec1[1] = 100;
        let name = "Evol";
        println!("Your name is {:?}", name);
        // name was dropped
    }
    println!("vec1={:?}", vec1);
}
