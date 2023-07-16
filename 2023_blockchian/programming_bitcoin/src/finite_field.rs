#![allow(dead_code)]

use std::{
    cmp::PartialEq,
    fmt,
    ops::{Add, Div, Mul, Sub},
};

#[derive(Debug, Clone, Copy)]
pub struct FiniteField {
    num: i32,
    prime: i32,
}

impl FiniteField {
    pub fn new(prime: i32, num: i32) -> Option<Self> {
        if !is_prime(prime) {
            // return Err("not a prime");
            return None;
        }

        if num.abs() >= prime {
            // return Err("num is too large");
            return None;
        }

        // return Ok(Self { num, prime });
        Some(Self { prime, num })
    }

    pub fn tuple(prime: i32, num1: i32, num2: i32) -> Option<(Self, Self)> {
        if !is_prime(prime) {
            return None;
        }

        if num1.abs() >= prime || num2.abs() >= prime {
            return None;
        }

        Some((Self { prime, num: num1 }, Self { prime, num: num2 }))
    }

    pub fn define(&self) {
        for n in 0..self.prime {
            assert!((n + self.num) % self.prime < self.prime);
        }

        assert!(-self.num % self.prime < self.prime);
    }

    pub fn pow(&self, mut n: u32) -> Self {
        // Self { num: self.num.pow(n) % self.prime, prime: self.prime }
        let mut num = 1;

        n = n % (self.prime as u32 - 1);
        for _ in 1..=n {
            num = (num * self.num) % self.prime;
        }

        Self { num, prime: self.prime }
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

        // Fermat's little theorem, p is a prime and n%p != 0: n^(p-1)%p=1
        /*
        let x = (self.prime - 2) as u32;
        let num = (self.num * other.num.pow(x)) % self.prime;
        Some(Self { num, prime: self.prime })
        */

        let mut ans = 1;
        for _ in 1..=(self.prime - 2) {
            ans = (ans % self.prime) * (other.num % self.prime);
        }

        Some(Self { num: (self.num * ans) % self.prime, prime: self.prime })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FFS {
    prime: i32,
}

impl FFS {
    pub fn new(prime: i32) -> Result<Self, &'static str> {
        if !is_prime(prime) {
            return Err("not a prime");
        }

        Ok(Self { prime })
    }

    pub fn vec(&self) -> Vec<i32> {
        (0..self.prime).collect::<Vec<_>>().into()
    }

    pub fn multipy(&self, m: i32) -> Vec<i32> {
        let mut vec: Vec<i32> = self.vec().iter().map(|v| (v * m) % self.prime).collect();
        vec.sort();

        vec
    }

    fn num(&self, num: i32) -> i32 {
        num % self.prime
    }

    pub fn add(&self, num1: i32, num2: i32) -> i32 {
        (self.num(num1) + self.num(num2)) % self.prime
    }

    pub fn sub(&self, num1: i32, num2: i32) -> i32 {
        (self.num(num1) - self.num(num2)) % self.prime
    }

    pub fn mul(&self, num1: i32, num2: i32) -> i32 {
        (self.num(num1) * self.num(num2)) % self.prime
    }

    pub fn div(&self, num1: i32, num2: i32) -> i32 {
        let mut ans = 1;
        for _ in 1..=(self.prime - 2) {
            ans = (ans % self.prime) * (num2 % self.prime);
        }

        (num1 * ans) % self.prime
    }

    pub fn pow(&self, num1: i32, mut n: u32) -> i32 {
        let mut ans = 1;

        n = n % (self.prime as u32 - 1);
        for _ in 1..=n {
            ans = (ans * num1) % self.prime;
        }

        ans
    }
}

pub fn is_prime(n: i32) -> bool {
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
        let num1 = FiniteField::new(11, 8).unwrap();
        let num2 = FiniteField::new(11, 8).unwrap();
        assert_eq!(num1, num2);

        let num2 = FiniteField::new(11, 5).unwrap();
        assert_eq!((num1 + num2), FiniteField::new(11, 2));
        assert_eq!((num1 - num2), FiniteField::new(11, 3));
        assert_eq!(num1.pow(3).num, 6);

        let ans = (num1 * num2).unwrap();
        assert_eq!(ans.num, 7);
        assert_eq!((ans / num1), Some(num2));
        assert_eq!((ans / num2), Some(num1));
    }

    #[test]
    fn t_ffs() {
        let set1 = FFS::new(13).unwrap();
        assert_eq!(set1.vec(), set1.multipy(6));
    }
}
