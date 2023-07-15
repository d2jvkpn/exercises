#![allow(dead_code)]

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
struct EC {
    a: i32,
    b: i32,
}

#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl EC {
    // bitcoin: a=0, b=7, curve secp256k1
    pub fn new(a: i32, b: i32) -> Self {
        Self { a, b }
    }

    pub fn on_curve(&self, p: &Point) -> bool {
        p.y.pow(2) == p.x.pow(3) + self.a * p.x + self.b
    }

    pub fn point_addition(p1: &Point, p2: &Point) -> Option<Point> {
        if p1.x == p2.x {
            return None;
        }

        let s = (p1.y - p2.y) / (p1.x - p2.x);
        let x = s.pow(2) - p1.x - p2.x;

        Some(Point { x, y: s * (p1.x - x) - p1.y })
    }
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl fmt::Display for EC {
    fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
        write!(w, "EC: y^2 = x^3 + {}*x + {}", self.a, self.b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_ec() {
        let ec = EC::new(5, 7);
        for p in [Point::new(2, 4), Point::new(-1, -1), Point::new(18, 77), Point::new(5, 7)] {
            println!("{}, {:?}: {}", ec, p, ec.on_curve(&p));
        }

        let p1 = Point::new(2, 5);
        let p2 = Point::new(-1, 1);
        let p3 = EC::point_addition(&p1, &p2).unwrap();
        println!("{:?}", p3);
        assert_eq!(p3, Point::new(0, -3));
    }
}
