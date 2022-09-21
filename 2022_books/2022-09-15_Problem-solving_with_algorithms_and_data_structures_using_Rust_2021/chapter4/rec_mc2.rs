// rec_mc2.rc

use std::{env, process};

fn rec_mc2(cashes: &[u32], amount: u32) -> Option<Vec<u32>> {
    if cashes.contains(&amount) {
        return Some(vec![amount]);
    }

    let mut c: u32 = 0;
    let mut min: Vec<u32> = Vec::new();

    for i in 0..cashes.len() - 1 {
        if amount < cashes[i] {
            continue;
        }
        let remain = amount - cashes[i];

        println!("~~~ {:?}, {}", &cashes[i + 1..], remain);
        if let Some(v) = rec_mc2(&cashes[i + 1..], remain) {
            if min.len() == 0 || min.len() > v.len() {
                min = v;
                c = cashes[i];
            }
        }
    }

    // println!("==> {:?}", min);
    if min.len() > 0 {
        min.push(c);
        Some(min)
    } else {
        None
    }
}

fn main() {
    let mut cashes = vec![1, 5, 10, 20, 50];
    cashes.sort_by(|a, b| b.cmp(a));

    let args: Vec<String> = env::args().collect();
    println!("~~~ args: {:?}, cashes: {:?}", args, cashes);

    let amount = args[1].parse::<u32>().unwrap();

    let refund = match rec_mc2(&cashes[..], amount) {
        Some(v) => v,
        None => {
            eprintln!("!!! can't refund");
            process::exit(1);
        }
    };

    println!(">>> need refund {} cashes: {:?}", refund.len(), refund);
}
