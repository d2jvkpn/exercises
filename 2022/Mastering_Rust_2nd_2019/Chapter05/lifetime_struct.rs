//  lifetime_struct.rs

struct Number<'a> {
    num: &'a u8,
}

impl Number<'_> {
    fn num(&self) -> u8 {
        *self.num
    }
}

fn main() {
    let n = Number { num: &8 };

    println!("Number.num = {}, {}", n.num, n.num());
}
