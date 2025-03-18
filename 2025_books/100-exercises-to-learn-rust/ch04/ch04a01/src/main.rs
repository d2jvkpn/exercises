fn main() {
    let x = WrappingU32 { inner: 5 };
    assert!(!x.is_zero());

    assert!(4.is_even());

    let y = WrappingU32 { inner: 5 };
    assert!(x == y);

    // destructing
    let WrappingU32 { inner } = x;
    println!("{inner}");

    let WrappingU32 { inner: the_inner } = x;
    println!("{the_inner}");
}

struct Ticket {
    title: String,
    description: String,
    status: String,
}

trait MaybeZero {
    fn is_zero(&self) -> bool;
}

struct WrappingU32 {
    inner: u32,
}

impl MaybeZero for WrappingU32 {
    fn is_zero(&self) -> bool {
        self.inner == 0
    }
}

trait IsEven {
    fn is_even(&self) -> bool;
}

impl IsEven for u32 {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }
}

impl PartialEq for WrappingU32 {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}
