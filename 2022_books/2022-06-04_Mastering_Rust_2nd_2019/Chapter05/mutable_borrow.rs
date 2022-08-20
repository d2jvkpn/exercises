// mutable_borrow.rs

fn main() {
    let mut a = String::from("Owned string");
    let mut a_ref = &mut a;
    a_ref.push('!');
}
