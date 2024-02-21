#![allow(dead_code)]

use procedural::*;

fn main() {
    //
    println!("\n==> proc_macro");
    do_nothing!(println!("Hello, world!"));

    log_info!([TIME] starting program ...);

    info!(starting program ...);

    //
    println!("\n==> proc_macro_derive");
    let mut db = Database::new("localhost://5432");
    db.connect();
    db.connections = 99;
    db.connect();

    //
    println!("\n==> proc_macro_attribute");
    let product = Product { name: "Apple".into(), price: 10 };
    buy_product(&product, 1);
}

/// Log Trait
trait Log {
    fn info(&self, msg: &str);
    fn warn(&self, msg: &str);
    fn error(&self, msg: &str);
}

#[derive(Log, Debug)]
struct Database {
    url: String,
    connections: u32,
}

impl Database {
    fn new(url: &str) -> Self {
        Self { url: url.to_string(), connections: 0 }
    }

    fn connect(&mut self) {
        self.connections += 1;

        if self.connections < 100 {
            self.info("new connection open.");
        } else {
            self.warn(format!("too many connections: {}", self.connections).as_ref());
        }
    }
}

///
#[derive(Debug)]
struct Product {
    name: String,
    price: u32,
}

#[log_call(verbose)]
fn buy_product(product: &Product, discount: u32) {
    // ...
}
