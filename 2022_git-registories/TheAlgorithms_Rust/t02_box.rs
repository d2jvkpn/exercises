use std::fmt;

#[derive(Debug)]
struct Data {
    pub value: usize,
}

impl Data {
    pub fn new(value: usize) -> Self {
        Self { value }
    }
}

impl Default for Data {
    fn default() -> Self {
        Data { value: 0 }
    }
}

impl fmt::Display for Data {
    fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
        write!(w, "value={}", self.value)
    }
}

fn main() {
    // println!("Hello, wrold!");

    let mut v1 = Box::new(Data::new(1));
    (*v1).value = 42;

    println!("==> {}, {:?}", v1, v1);
}
