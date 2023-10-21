fn main() {
    // println!("Hello, world!");

    let mut income: f32 = 100.0;
    apply_tax(&mut income);

    println!("==> income: {income}");
}

fn apply_tax(income: &mut f32) {
    let tax: f32 = 0.1;

    *income = *income * (1.0 - tax);
}
