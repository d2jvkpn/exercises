#![allow(dead_code)]

use std::{
    cmp::PartialEq,
    fmt,
    ops::{Add, Div, Mul, Sub},
};

#[derive(Debug, Clone, Copy)]
struct FiniteField {
    num: i32,
    prime: i32,
}

impl FiniteField {
    pub fn new(num: i32, prime: i32) -> Result<Self, &'static str> {
        if !is_prime(prime) {
            return Err("not a prime");
        }

        if num.abs() >= prime {
            return Err("num is too large");
        }

        return Ok(Self { num, prime });
    }

    pub fn define(&self) {
        for n in 0..self.prime {
            assert!((n + self.num) % self.prime < self.prime);
        }

        assert!(-self.num % self.prime < self.prime);
    }

    pub fn pow(&self, n: u32) -> Self {
        Self { num: self.num.pow(n) % self.prime, prime: self.prime }
    }
}

impl fmt::Display for FiniteField {
    fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
        write!(w, "FieldElement_{}({})", self.prime, self.num)
    }
}

impl PartialEq for FiniteField {
    fn eq(&self, other: &Self) -> bool {
        self.prime == other.prime && self.num == other.num
    }
}

impl Add for FiniteField {
    type Output = Option<Self>;

    fn add(self, other: Self) -> Self::Output {
        if self.prime != other.prime {
            return None;
        }

        Some(Self { num: (self.num + other.num) % self.prime, prime: self.prime })
    }
}

impl Sub for FiniteField {
    type Output = Option<Self>;

    fn sub(self, other: Self) -> Self::Output {
        if self.prime != other.prime {
            return None;
        }

        Some(Self { num: (self.num - other.num) % self.prime, prime: self.prime })
    }
}

impl Mul for FiniteField {
    type Output = Option<Self>;

    fn mul(self, other: Self) -> Self::Output {
        if self.prime != other.prime {
            return None;
        }

        Some(Self { num: (self.num * other.num) % self.prime, prime: self.prime })
    }
}

impl Div for FiniteField {
    type Output = Option<Self>;

    fn div(self, other: Self) -> Self::Output {
        if self.prime != other.prime {
            return None;
        }

        // Fermat's little theorem
        let x = (self.prime - 2) as u32;
        let num = (self.num * other.num.pow(x)) % self.prime;
        Some(Self { num, prime: self.prime })
    }
}

struct FiniteFieldSet {
    vec: Vec<i32>,
    prime: i32,
}

impl FiniteFieldSet {
    pub fn new(prime: i32) -> Result<Self, &'static str> {
        if !is_prime(prime) {
            return Err("not a prime");
        }

        Ok(Self { vec: (0..prime).collect::<Vec<_>>().into(), prime })
    }

    pub fn multipy(&self, m: i32) -> Self {
        let mut vec: Vec<i32> = self.vec.iter().map(|v| (v * m) % self.prime).collect();
        vec.sort();

        Self { vec, prime: self.prime }
    }
}

fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }

    for a in 2..n {
        if n % a == 0 {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_finite_field() {
        let num1 = FiniteField::new(8, 11).unwrap();
        let num2 = FiniteField::new(8, 11).unwrap();
        assert_eq!(num1, num2);

        let num2 = FiniteField::new(5, 11).unwrap();
        assert_eq!((num1 + num2).unwrap().num, 2);
        assert_eq!((num1 - num2).unwrap().num, 3);
        assert_eq!(num1.pow(3).num, 6);

        let ans = (num1 * num2).unwrap();
        assert_eq!(ans.num, 7);
        assert_eq!((ans / num1).unwrap().num, num2.num);
        assert_eq!((ans / num2).unwrap().num, num1.num);
    }

    #[test]
    fn t_finite_fields_set() {
        let set1 = FiniteFieldSet::new(13).unwrap();
        let set2 = set1.multipy(6);
        assert_eq!(set1.vec, set2.vec);
    }
}
