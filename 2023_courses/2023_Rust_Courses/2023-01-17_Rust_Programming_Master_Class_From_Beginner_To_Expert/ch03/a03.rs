fn main() {
    let mut heap_num = vec![4, 5, 6];
    let ref1 = &mut heap_num;
    // let ref2 = &mut heap_num;
    // println!("ref1: {:?}, ref2: {:?}", ref1, ref2);
    println!("ref1: {ref1:?}");

    let ref1 = &heap_num;
    let ref2 = &heap_num;
    println!("ref1: {ref1:?}, ref2: {ref2:?}");

    let ref3 = &mut heap_num; // both ref1 and ref2 are not avaiable
    println!("ref3: {ref3:?}");

    heap_num.push(7); // can't access ref1, ref2, ref3
    println!("heap_num: {heap_num:?}");
}
