#![allow(dead_code)]

use crate::finite_field::{FiniteField, FiniteFieldSet};
use std::{convert::From, fmt};

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

    pub fn point_addition(&self, p1: &Point, p2: &Point) -> Option<Point> {
        if !self.on_curve(&p1) || !self.on_curve(&p2) {
            return None;
        }

        if p1.x == p2.x && p1.y != p2.y {
            return None;
        }

        let s = if p1 == p2 {
            (3 * p1.x.pow(2) + self.a) / 2 * p1.y
        } else {
            (p1.y - p2.y) / (p1.x - p2.x)
        };

        let x = s.pow(2) - p1.x - p2.x;

        Some(Point { x, y: s * (p1.x - x) - p1.y })
    }

    pub fn finite_field_v1(&self, prime: i32, point: &Point) -> Option<bool> {
        let (x, y) = FiniteField::tuple(prime, point.x, point.y)?;
        let (a, b) = FiniteField::tuple(prime, self.a, self.b)?;

        let ans = (a * x)?;
        let ans = (x.pow(3) + ans)?;
        let ans = (ans + b)?;
        Some(y.pow(2) == ans)
    }

    pub fn finite_field(&self, prime: i32, point: &Point) -> Option<bool> {
        let ffs = FiniteFieldSet::new(prime).ok()?;

        let ans = ffs.add(ffs.pow(point.x, 3), ffs.mul(self.a, point.x));
        Some(ffs.pow(point.y, 2) == ffs.add(ans, self.b))
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

impl From<(i32, i32)> for Point {
    fn from(input: (i32, i32)) -> Self {
        Self { x: input.0, y: input.1 }
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
        let p2 = Point::new(-1, -1);
        let p3 = ec.point_addition(&p1, &p2).unwrap();
        println!("Point Addition: {:?} + {:?} = {:?}", p1, p2, p3);
        assert!(ec.on_curve(&p3));

        let p1 = Point::new(-1, -1);
        let p3 = ec.point_addition(&p1, &p1).unwrap();
        assert!(ec.on_curve(&p3));
        println!("Point Addition: {:?} + {:?} = {:?}", p1, p1, p3);
    }

    #[test]
    fn t_spec265k1() {
        let spec265k1 = EC::new(0, 7);
        let prime = 223;

        for p in [(192, 105), (17, 56), (1, 193)] {
            let point = Point::new(p.0, p.1);
            let ans = spec265k1.finite_field(prime, &point).unwrap();
            assert!(ans);
            println!("prime = {}, point={:?}, {}", prime, point, ans);
        }

        for p in [(200, 119), (42, 99)] {
            let point = Point::new(p.0, p.1);
            let ans = spec265k1.finite_field(prime, &point).unwrap();
            assert!(!ans);
            println!("prime = {}, point={:?}, {}", prime, point, ans);
        }
    }
}
