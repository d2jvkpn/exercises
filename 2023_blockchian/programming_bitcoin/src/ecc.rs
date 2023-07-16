#![allow(dead_code)]

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
struct EC {
    a: f32,
    b: f32,
}

#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: f32,
    y: f32,
}

impl EC {
    // bitcoin: a=0, b=7, curve secp256k1
    pub fn new(a: f32, b: f32) -> Self {
        Self { a, b }
    }

    pub fn on_curve(&self, p: &Point) -> bool {
        p.y.powf(2.0) == p.x.powf(3.0) + self.a * p.x + self.b
    }

    pub fn point_addition(&self, p1: &Point, p2: &Point) -> Option<Point> {
        if !self.on_curve(&p1) || !self.on_curve(&p2) {
            return None;
        }

        if p1.x == p2.x && p1.y != p2.y {
            return None;
        }

        let s = if p1.y == p2.y {
            (3.0 * p1.x.powf(2.0) + self.a) / 2.0 * p1.y
        } else {
            (p1.y - p2.y) / (p1.x - p2.x)
        };

        let x = s.powf(2.0) - p1.x - p2.x;

        Some(Point { x, y: s * (p1.x - x) - p1.y })
    }
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
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
        let ec = EC::new(5.0, 7.0);
        for p in [
            Point::new(2.0, 4.0),
            Point::new(-1.0, -1.0),
            Point::new(18.0, 77.0),
            Point::new(5.0, 7.0),
        ] {
            println!("{}, {:?}: {}", ec, p, ec.on_curve(&p));
        }

        let p1 = Point::new(2.0, 5.0);
        let p2 = Point::new(-1.0, -1.0);
        let p3 = ec.point_addition(&p1, &p2).unwrap();
        println!("Point Addition: {:?} + {:?} = {:?}", p1, p2, p3);
        assert!(ec.on_curve(&p3));

        let p1 = Point::new(-1.0, -1.0);
        let p3 = ec.point_addition(&p1, &p1).unwrap();
        assert!(ec.on_curve(&p3));
        println!("Point Addition: {:?} + {:?} = {:?}", p1, p1, p3);
    }
}
