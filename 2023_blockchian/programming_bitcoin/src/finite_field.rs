#![allow(dead_code)]

use std::{
    cmp::PartialEq,
    fmt,
    ops::{Add, AddAssign, Div, Mul, Sub},
};

// % is remainder, not modulus: assert_eq!(-226 % 223, -3);
pub fn modulus(a: i32, b: i32) -> i32 {
    ((a % b) + b) % b
}

#[derive(Debug, Clone, Copy)]
pub struct FiniteField {
    pub num: i32,
    pub prime: i32, // order of a FiniteField
}

impl FiniteField {
    pub fn new(num: i32, prime: i32) -> Option<Self> {
        if !is_prime(prime) {
            // return Err("not a prime");
            return None;
        }

        if num.abs() >= prime {
            // return Err("num is too large");
            return None;
        }

        // return Ok(Self { num, prime });
        Some(Self { num, prime })
    }

    pub fn sibling(&self, num: i32) -> Self {
        Self { prime: self.prime, num }
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
            // assert!((n + self.num) % self.prime < self.prime);
            assert!(modulus(n + self.num, self.prime) < self.prime);
        }

        // assert!(-self.num % self.prime < self.prime);
        assert!(modulus(-self.num, self.prime) < self.prime);
    }

    pub fn pow(&self, mut n: u32) -> Self {
        // Self { num: modulus(self.num.pow(n), self.prime), prime: self.prime }
        let mut num = 1;

        n = n % (self.prime as u32 - 1);
        for _ in 1..=n {
            num = modulus(num * self.num, self.prime);
        }

        Self { num, prime: self.prime }
    }

    pub fn rmul(&self, coefficient: u32) -> Self {
        /*
        let mut ans = self.clone();
        for _ in 2..=coefficient {
            ans = ans + *self;
        }
        ans
        */

        // TODO: model
        let mut coef = coefficient;
        let mut current = *self;
        let mut result = self.sibling(0);

        while coef > 0 {
            if coef & 1 == 1 {
                result += current;
            }

            current += current;
            coef >>= 1;
        }

        result
    }
}

impl fmt::Display for FiniteField {
    fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
        write!(w, "FieldElement(p={}, v={})", self.prime, self.num)
    }
}

impl PartialEq for FiniteField {
    fn eq(&self, other: &Self) -> bool {
        self.prime == other.prime && self.num == other.num
    }
}

/*
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
*/

impl Add for FiniteField {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self { num: (self.num + other.num) % self.prime, prime: self.prime }
    }
}

impl AddAssign for FiniteField {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Sub for FiniteField {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self { num: modulus(self.num - other.num, self.prime), prime: self.prime }
    }
}

impl Mul for FiniteField {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self { num: modulus(self.num * other.num, self.prime), prime: self.prime }
    }
}

impl Div for FiniteField {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        // Fermat's little theorem, p is a prime and n%p != 0: n^(p-1)%p=1
        /*
        let x = (self.prime - 2) as u32;
        let num = (self.num * other.num.pow(x)) % self.prime;
        Some(Self { num, prime: self.prime })
        */

        let mut ans = 1;
        for _ in 1..=(self.prime - 2) {
            ans = modulus(ans, self.prime) * (other.num % self.prime);
        }

        Self { num: modulus(self.num * ans, self.prime), prime: self.prime }
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
        let num1 = FiniteField::new(8, 11).unwrap();
        let num2 = FiniteField::new(8, 11).unwrap();
        assert_eq!(num1, num2);

        let num2 = FiniteField::new(5, 11).unwrap();
        assert_eq!((num1 + num2), num1.sibling(2));
        assert_eq!((num1 - num2), num1.sibling(3));
        assert_eq!(num1.pow(3).num, 6);

        let ans = num1 * num2;
        assert_eq!(ans.num, 7);
        assert_eq!((ans / num1), num2);
        assert_eq!((ans / num2), num1);

        let num1 = FiniteField::new(6, 7).unwrap();
        assert_eq!(num1.rmul(2), num1.sibling(5));
        assert_eq!(num1.rmul(3), num1.sibling(4));

        let num1 = FiniteField::new(2, 19).unwrap();
        assert_eq!(num1 / num1.sibling(7), num1.sibling(3));
        assert_eq!(num1.sibling(3) * num1.sibling(7), num1);

        let num1 = FiniteField::new(7, 19).unwrap();
        assert_eq!(num1 / num1.sibling(5), num1.sibling(9));
        assert_eq!(num1.sibling(5) * num1.sibling(9), num1);

        let num1 = FiniteField::new(221, 223).unwrap();
        let num2 = num1.sibling(170);
        let num3 = num1.sibling(60);
        assert_eq!(num1.pow(2) - num2 - num3, num1.sibling(220));
    }

    /*
    #[test]
    fn t_ffs() {
        let set1 = FFS::new(13).unwrap();
        assert_eq!(set1.vec(), set1.multipy(6));
    }
    */
}
