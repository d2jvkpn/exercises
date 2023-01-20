fn main() {
    //
    let vec1 = vec![45, 30, 85, 90, 41, 39];

    for v in vec1 {
        // vec1.into_iter(), vec1 was moved
        println!("v = {}", v); // i32
    }

    //
    let mut vec1 = vec![45, 30, 85, 90, 41, 39];

    for v in &vec1 {
        // vec1.iter(), vec1 was borrowed
        println!("v = {}", v); // &i32
    }
    println!("vec1 = {:?}", vec1);

    for (i, v) in vec1.iter().enumerate() {
        println!("vec1[{}] = {}", i, v); // usize, &i32
    }
    println!("vec1 = {:?}", vec1);

    for v in vec1.iter_mut() {
        *v += 1; // v: &mut i32
    }
    println!("vec1 = {:?}", vec1);
}
