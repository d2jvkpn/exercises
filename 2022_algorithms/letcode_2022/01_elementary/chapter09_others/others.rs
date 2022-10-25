pub fn hamming_weight(n: u32) -> i32 {
    let mut count: i32 = 0;
    let mut val = n;

    while val > 0 {
        if val & 1 == 1 {
            count += 1;
        }

        val >>= 1;
    }
    count
}

pub fn hamming_distance(x: i32, y: i32) -> i32 {
    hamming_weight((x ^ y) as u32)
}

pub fn reverse_bits(x: u32) -> u32 {
    let mut rev: u32 = 0;
    let mut n = x;

    for _ in 0..32 {
        rev <<= 1;
        if n & 1 == 1 {
            rev += 1;
        }
        n >>= 1;
    }

    rev
}

fn pascal_triangle(num_rows: i32) -> Vec<Vec<i32>> {
    let (r1, r2) = (vec![1], vec![1, 1]);

    match num_rows {
        1 => return vec![r1],
        2 => return vec![r1, r2],
        _ => {}
    }

    let mut output = Vec::new();
    output.push(r1);
    output.push(r2);

    let num_rows = num_rows as usize;

    for n in 3..=num_rows {
        let last = &output[output.len() - 1];
        let mut row = Vec::with_capacity(n);

        row.push(1);
        last[1..].iter().enumerate().for_each(|(idx, val)| row.push(last[idx] + val));
        row.push(*last.last().unwrap());

        output.push(row);
    }

    output
}

pub fn is_valid(s: String) -> bool {
    if s.len() % 2 != 0 {
        return false;
    }

    fn is_pair(p: (char, char)) -> bool {
        match p {
            ('(', ')') | ('{', '}') | ('[', ']') => true,
            _ => false,
        }
    }

    let mut chars = Vec::with_capacity(s.len());

    for v in s.chars() {
        let l = chars.len();

        if l == 0 || !is_pair((chars[l - 1], v)) {
            chars.push(v);
        } else {
            _ = chars.pop();
        }
        // println!("   {:?}", chars);
    }

    chars.len() == 0
}

pub fn missing_number(nums: Vec<i32>) -> i32 {
    let len = nums.len() + 1;
    let mut bools = Vec::with_capacity(len);
    bools.resize(len, false);

    nums.iter().for_each(|&v| {
        let idx = v as usize;
        if idx < bools.len() {
            bools[idx] = true;
        }
    });

    println!("    {:?}", bools);

    match bools.iter().position(|&v| !v) {
        Some(v) => v as i32,
        None => -1,
    }
}

fn main() {
    println!(">>> hamming_weight: {}", hamming_weight(0b00000000000000000000000000001011u32));

    println!(">>> hamming_distance: {}", hamming_distance(1, 4));

    println!(">>> pascal_triangle: {:?}", pascal_triangle(5));

    println!(">>> is_valid: {}", is_valid("{}".to_string()));

    println!(">>> missing_number: {:?}", missing_number(vec![3, 0, 1]));
    println!(">>> missing_number: {:?}", missing_number(vec![0, 1]));
}
