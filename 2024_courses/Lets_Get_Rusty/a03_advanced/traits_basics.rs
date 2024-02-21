#![allow(dead_code)]

fn main() {
    // println!("Hello, wrold!");
    let car = Car { name: "Honda".into(), model: "Civic".into(), year: 1995 };
    let truck = Truck { make: "Honda".into(), model: "Civic".into(), year: 2006 };

    do_park(&car);
    do_park(&truck);
}

struct Car {
    name: String,
    model: String,
    year: u16,
}

struct Truck {
    make: String,
    model: String,
    year: u16,
}

impl Truck {
    fn unload(&self) -> String {
        "Unloading".to_owned()
    }
}

trait Park {
    // fn park(&self);

    fn park(&self) {
        println!("==> Parking...")
    }
}

// default
impl Park for Car {}

impl Park for Truck {
    fn park(&self) {
        println!("==> {}, parking the truck.", self.unload());
    }
}

// fn do_park(item: &impl Park) {
// T: Park + Paintable
fn do_park<T: Park>(item: &T) {
    item.park();
}

fn create_vehicle_a01() -> impl Park {
    Car { name: "Honda".into(), model: "Civic".into(), year: 1995 }
}

fn create_vehicle_a02() -> Box<dyn Park> {
    let car = Car { name: "Honda".into(), model: "Civic".into(), year: 1995 };
    Box::new(car)
}
