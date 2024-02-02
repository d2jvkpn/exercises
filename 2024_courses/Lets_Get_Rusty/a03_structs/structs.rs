#![allow(dead_code)]
#![allow(unused_variables)]

use std::fmt;

fn main() {
    // structs
    // let product = Product::new("Hello".to_string(), 42.0).with_in_stack(true);

    let mut product: Product = Default::default();
    product.set_name("Hello").set_price(42.25).set_in_stack(true);

    println!("~~~ {}", product);
    let num = 3;
    println!("==> You need to pay ${} for {} {:?}.", product.pay(num), num, product.name);

    // tuples structs
    let rbg_color: (i32, i32, i32) = (256, 106, 0);
    let cmyk_color: (i32, i32, i32, i32) = (0, 58, 100, 0);

    #[derive(Debug)]
    struct RGB(i32, i32, i32);

    #[derive(Debug)]
    struct CMYK(i32, i32, i32, i32);

    let color1 = RGB(256, 106, 0);
    let color2 = CMYK(0, 58, 100, 0);
    println!("~~~ color1: {:?}, color2: {color2:?}", color1);

    // unit-like structs
    struct MyStruct;

    // enum
    let category = Category::Clothing;
    dbg!(&category);

    let cmd = Command::Undo;
    let cmd = Command::AddText(String::from("Test"));
    let cmd = Command::Replace { from: "a".to_string(), to: "b".to_string() };

    println!("==> cmd: {:?}, serilize: {}", cmd, cmd.serialize());
}

#[derive(Debug)]
struct Product {
    name: String,
    price: f32,
    in_stock: bool,
}

impl Product {
    pub fn new(name: String, price: f32) -> Self {
        Self { name, price, in_stock: false }
    }

    pub fn default_sales_tax() -> f32 {
        0.1
    }

    pub fn with_in_stack(mut self, in_stack: bool) -> Self {
        self.in_stock = in_stack;
        self
    }

    pub fn set_name(&mut self, name: &str) -> &mut Self {
        self.name = name.to_string();
        self
    }

    pub fn set_price(&mut self, price: f32) -> &mut Self {
        self.price = price;
        self
    }

    pub fn set_in_stack(&mut self, in_stock: bool) -> &mut Self {
        self.in_stock = in_stock;
        self
    }

    pub fn sales_tax(&self) -> f32 {
        self.price * Self::default_sales_tax()
    }

    pub fn pay(&self, num: usize) -> f32 {
        self.price * (num as f32)
    }
}

impl fmt::Display for Product {
    fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
        write!(
            w,
            "Product {{name: {:?}, price: {}, in_stock: {}}}",
            self.name, self.price, self.in_stock,
        )
    }
}

impl Default for Product {
    fn default() -> Self {
        Self { name: "".to_string(), price: 0.0, in_stock: false }
    }
}

#[derive(Debug)]
enum Category {
    Book,
    Clothing,
    Electrics,
    Other,
}

#[derive(Debug)]
enum Command {
    Undo,
    Redo,
    AddText(String),
    MoveCursor(i32, i32),
    Replace { from: String, to: String },
}

impl Command {
    fn serialize(&self) -> String {
        match self {
            Self::Undo => "undo".into(),
            Self::Redo => "redo".into(),
            Self::AddText(v) => format!("add_text({:?})", v),
            Self::MoveCursor(ref x, ref y) => format!("move_coursor({}, {})", x, y),
            Self::Replace { ref from, to } => {
                format!(r#"{{"replace":{{"from":{:?},"to":{:?}}}}}"#, from, to)
            }
        }
    }
}
