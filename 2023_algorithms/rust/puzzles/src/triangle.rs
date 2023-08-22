fn main() {
    let vec = vec![1, 2, 3, 4, 5, 6];

    triangle(&vec);
}

fn triangle(slice: &[i32]) {
    if slice.len() < 1 {
        return;
    }

    let mut vec = Vec::with_capacity(slice.len() - 1);
    for i in 0..(slice.len() - 1) {
        vec.push(slice[i] + slice[i + 1]);
    }

    triangle(&vec);
    println!("{:?}", slice);
}
