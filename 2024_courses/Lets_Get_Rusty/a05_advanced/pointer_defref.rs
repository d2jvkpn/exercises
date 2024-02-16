use std::ops::{Deref, DerefMut};

fn main() {
    // println!("Hello, wrold!");

    let s1: SmartPointer<Box<String>> = SmartPointer::new(Box::new("Let's Get Rusty".to_owned()));

    let s2: &String = &(**s1);

    println!("~~~ s1.len(): {}", s1.len());
    println!("~~~ s2.len(): {}", s2.len());
}

struct SmartPointer<T> {
    value: T,
}

impl<T> SmartPointer<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }
}

impl<T> Deref for SmartPointer<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for SmartPointer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}
