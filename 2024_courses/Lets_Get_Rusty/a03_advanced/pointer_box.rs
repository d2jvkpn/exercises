#![allow(dead_code)]

use std::fmt::Debug;

fn main() {
    // println!("Hello, wrold!");

    let btn1 = Button { text: "button 01".to_owned() };

    let btn2 = Box::new(Button { text: "button 02".to_owned() });

    let btn3 = btn1; // Ownership of Button is tansfered.
    let btn4 = btn2; // Ownership of Box is tansfered as Button is allocated on heap.

    let components: Vec<Box<dyn UIComponent>> = vec![Box::new(btn3), btn4];
    println!("~~~ components: {components:?}");
}

//
#[derive(Debug)]
struct Button {
    text: String,
}

trait UIComponent: Debug {
    fn render(&self) {
        println!("==> Rending component...");
    }
}

impl UIComponent for Button {}

//
struct Container {
    name: String,
    child: Box<Container>,
}
