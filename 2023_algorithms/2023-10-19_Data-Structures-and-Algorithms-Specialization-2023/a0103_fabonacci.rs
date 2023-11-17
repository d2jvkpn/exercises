use std::io::{stdin, stdout, Write};

fn main() {
    let mut input = String::new();

    print!("==> Enter number: ");

    let _ = stdout().flush();
    input.clear();
    stdin().read_line(&mut input).expect("Failed to read line");
    let num: usize = input.trim().parse::<usize>().expect("Input not an integer");

    let ans = match num {
        1 => vec![0_u128],
        2 => vec![0, 1],
        _ => {
            let mut vec = Vec::with_capacity(num);
            vec.push(0);
            vec.push(1);

            let (mut p1, mut p2, mut val) = (0, 1, 0);

            (2..num).for_each(|_| {
                // let size = vec.len();
                // let (p1, p2) = (vec[size - 2], vec[size - 1]);
                // vec.push(p1 + p2);

                val = p1 + p2;
                vec.push(val);
                p1 = p2;
                p2 = val;
            });

            vec
        }
    };

    println!("==> ans: {:?}", ans);
}
