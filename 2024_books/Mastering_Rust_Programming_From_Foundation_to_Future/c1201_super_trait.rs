fn main() {
    let xx: XX = XX {};

    xx.print();
    xx.debug();
}

trait Printable {
    fn print(&self);
}

trait Debugable: Printable {
    fn debug(&self);
}

struct XX;

impl Printable for XX {
    fn print(&self) {
        println!("~~~ Printing XX");
    }
}

impl Debugable for XX {
    fn debug(&self) {
        println!("~~~ Debugging XX");
    }
}
