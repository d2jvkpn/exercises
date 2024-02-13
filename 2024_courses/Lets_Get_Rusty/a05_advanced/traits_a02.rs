#![allow(dead_code)]

fn main() {
    // println!("Hello, wrold!");

    let car = Car {};
    let house = House {};

    //
    let object1 = create_paintable_1(); // ?? return type

    paint_red_1(&car);
    paint_red_1(&house);
    paint_red_1(&object1);

    //
    let object2: Box<dyn Paint> = create_paintable_2(true);
    paint_read_2(object2.as_ref()); // as_ref convert Box<dyn Paint> to &dyn Paint

    let _paintables: Vec<&dyn Paint> = vec![&car, &house, object2.as_ref()];
}

// static dispath
fn create_paintable_1() -> impl Paint {
    House {}
}

fn paint_red_1<T: Paint>(object: &T) {
    print!("==> paint_red_1: ");
    object.paint("red");
}

// dynamic dispath
fn create_paintable_2(vehicle: bool) -> Box<dyn Paint> {
    if vehicle {
        Box::new(Car {})
    } else {
        Box::new(House {})
    }
}

fn paint_read_2(object: &dyn Paint) {
    print!("==> paint_red_2: ");
    object.paint("red");
}

// trait Vehicle: Paint + AnotherTrait {
trait Vehicle: Paint {
    fn park(&self);

    fn get_default_color() -> String {
        "black".to_owned()
    }
}

trait Paint {
    fn paint(&self, color: &str) {
        println!("painting object: {}", color);
    }
}

//
struct Car {}

impl Vehicle for Car {
    fn park(&self) {
        println!("parking car!");
    }
}

impl Paint for Car {}

//
struct Truck {}

impl Vehicle for Truck {
    fn park(&self) {
        println!("parking truck");
    }
}

impl Paint for Truck {}

//
struct House {}

impl Paint for House {
    fn paint(&self, color: &str) {
        println!("painting house: {}", color);
    }
}
