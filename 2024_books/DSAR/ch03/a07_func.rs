fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    let sum_of_squares: i32 = numbers.into_iter().filter(|&x| x % 2 != 0).map(|x| x * x).sum();

    println!("Sum of squares of odd numbers: {}", sum_of_squares);
}
