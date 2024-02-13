use std::fmt;

fn main() {
    // println!("Hello, wrold!");

    let p1 = Point::new(3, 1);
    let mut p2: Point = Default::default();
    (p2.x, p2.y) = (p1.x, p1.y);
    let p3 = Point::new(5, 5);

    println!("~~~ p1: {:?}, p1=p2: {}, p1=p3: {}", p1, p1 == p2, p1 == p3);

    let p4: PointWrapper = p2.into();
    let p5: PointWrapper = p3.into();
    println!("~~~ p4: {p4}, p5: {p5}");

    p4.as_ref().hello();
}

#[derive(Debug, Default, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn hello(&self) {
        println!("==> Hello Point({}, {})", self.x, self.y);
    }
}

struct PointWrapper(Point);

impl PartialEq for PointWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl From<Point> for PointWrapper {
    fn from(p: Point) -> Self {
        Self(p)
    }
}

impl AsRef<Point> for PointWrapper {
    fn as_ref(&self) -> &Point {
        &self.0
    }
}

impl fmt::Display for PointWrapper {
    fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
        write!(w, "PointWrapper({}, {})", self.0.x, self.0.y)
    }
}
