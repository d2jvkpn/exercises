fn main() {
    let grade = get_grade(50);
    println!("grade = {grade}");
}

fn get_grade(val: i32) -> char {
    match val {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        _ => 'F',
    }
}
