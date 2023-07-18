#![allow(dead_code)]

use crate::finite_field::{is_prime, FiniteField};
use std::{convert::From, fmt};

#[derive(Debug, Clone)]
pub struct EC {
    a: i32,
    b: i32,
    prime: i32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl EC {
    // bitcoin: a=0, b=7, curve secp256k1
    pub fn new(a: i32, b: i32, prime: i32) -> Result<Self, &'static str> {
        if !is_prime(prime) {
            return Err("not a prime");
        }
        Ok(Self { a, b, prime })
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

        if p1 == p2 && p1.y == 0 {
            return None;
        }

        let s = if p1 == p2 {
            (3 * p1.x.pow(2) + self.a) / (2 * p1.y)
        } else {
            (p1.y - p2.y) / (p1.x - p2.x)
        };

        let x = s.pow(2) - p1.x - p2.x;

        Some(Point { x, y: s * (p1.x - x) - p1.y })
    }

    pub fn ff_on_curve(&self, point: &Point) -> Option<bool> {
        let (x, y) = FiniteField::tuple(self.prime, point.x, point.y)?;
        let (a, b) = FiniteField::tuple(self.prime, self.a, self.b)?;

        let ans = x.pow(3) + a * x + b;
        Some(y.pow(2) == ans)
    }

    pub fn add(&self, p1: Point, p2: Point) -> Option<Point> {
        if p1.x == i32::MAX || p1.y == i32::MAX {
            return Some(p2);
        } else if p2.x == i32::MAX || p2.y == i32::MAX {
            return Some(p1);
        }

        /*
        if self.ff_on_curve(&p1) != Some(true) || self.ff_on_curve(&p2) != Some(true) {
            return None;
        }
        */

        if p1.x == p2.x && p1.y != p2.y {
            return None;
        }

        if p1 == p2 && p1.y == 0 {
            return None;
        }

        let x1 = FiniteField::new(p1.x, self.prime).unwrap();
        let y1 = x1.sibling(p1.y);
        let x2 = x1.sibling(p2.x);
        let y2 = x1.sibling(p2.y);
        let a = x1.sibling(self.a);
        // let b = x1.sibling(self.b);

        let v2 = x1.sibling(2);
        let v3 = x1.sibling(3);

        let s = if p1 == p2 {
            // (3 * p1.x.pow(2) + self.a) / (2 * p1.y)
            (v3 * x1.pow(2) + a) / (v2 * y1)
        } else {
            // (p1.y - p2.y) / (p1.x - p2.x)
            (y2 - y1) / (x2 - x1)
        };

        // dbg!(&(s, x1, x2));
        let x = s.pow(2) - x1 - x2;
        let y = s * (x1 - x) - y1;

        Some(Point { x: x.num, y: y.num })
    }

    pub fn rmul(&self, p1: Point, coefficient: u32) -> Option<Point> {
        /*
        // time complexity: O(n)
        let mut ans = p1;
        for _ in 2..=coefficient {
            ans = self.add(ans, p1).unwrap();
        }

        ans
        */

        // Square-and-Multiply Chain, time complexity: O(log2n)
        let mut coef = coefficient;
        let mut current = p1;
        let mut ans = Point::new(i32::MAX, i32::MAX);

        while coef > 0 {
            if coef & 1 == 1 {
                ans = self.add(ans, current)?;
            }

            current = self.add(current, current)?;
            coef >>= 1;
        }

        Some(ans)
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
    fn t_point() {
        let ec = EC::new(5, 7, 2).unwrap();

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
        let spec265k1 = EC::new(0, 7, 223).unwrap();
        println!("{:?}", spec265k1);

        for p in [(192, 105), (17, 56), (1, 193), (47, 71), (143, 98)] {
            let point = Point::new(p.0, p.1);
            let ans = spec265k1.ff_on_curve(&point).unwrap();
            assert!(ans);
            println!("{}, {:?}, {}", spec265k1, point, ans);
        }

        for p in [(200, 119), (42, 99)] {
            let point = Point::new(p.0, p.1);
            let ans = spec265k1.ff_on_curve(&point).unwrap();
            assert!(!ans);
            println!("{}, {:?}, {}", spec265k1, point, ans);
        }
    }

    #[test]
    fn t_ecc() {
        let spec265k1 = EC::new(0, 7, 223).unwrap();
        println!("{:?}", spec265k1);

        let (p1, p2) = ((170, 142).into(), (60, 139).into());
        println!("{:?} + {:?} = {:?}", p1, p2, spec265k1.add(p1, p2));

        let (p1, p2) = ((47, 71).into(), (17, 56).into());
        println!("{:?} + {:?} = {:?}", p1, p2, spec265k1.add(p1, p2));

        let (p1, p2) = ((143, 98).into(), (76, 66).into());
        println!("{:?} + {:?} = {:?}", p1, p2, spec265k1.add(p1, p2));

        let p1 = (47, 71).into();
        let ans = spec265k1.add(p1, p1).unwrap();
        assert_eq!(ans, (36, 111).into());
        assert_eq!(spec265k1.rmul(p1, 3), Some((15, 137).into()));

        for s in 1..21 {
            println!("{:?} * {} = {:?}", p1, s, spec265k1.rmul(p1, s));
        }
    }
}
