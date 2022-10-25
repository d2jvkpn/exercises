fn main() {
    let mut arr1 = vec![1, 2, 3, 4, 5, 6];
    let arr2 = vec![9, 10, 11];
    println!(">>> nums1: {:?}, nums2: {:?}", arr1, arr2);

    arr2.iter().enumerate().for_each(|(i, v)| {
        println!("~~~ {}, {}", i, v);
        arr1[i] = *v;
    });

    arr1[3..].iter_mut().for_each(|v| {
        *v = *v + 1;
    });

    println!("~~~ {:?}", arr1);
}
