#![allow(dead_code)]

use crate::{ecc::*, finite_field::*};

const A: i32 = 0;
const B: i32 = 7;
// python3 chatgpt/trial_division.py
const P: i32 = 271;
const N: i32 = 251;
const G: Point = Point { x: 101, y: 237 };

fn rmul(point: Point, coefficient: u32) -> Option<Point> {
    let coef = modulus(coefficient as i32, N);
    let ec = EC::new(A, B, P).unwrap();
    ec.rmul(point, coef as u32)
}

fn pub_key(e: u32) -> Option<Point> {
    let ec = EC::new(A, B, P).unwrap();
    let ans = ec.rmul(G, e)?;
    Some(ans)
}

fn demo(e: u32, k: i32 /* a random numer */) -> (u32, u32, Point) {
    let ec = EC::new(A, B, P).unwrap();
    let pk = ec.rmul(G, e).unwrap();

    // let R = ec.rmul(G, k as u32).unwrap();
    // let r = R.x;

    // u*G + v*P = k*G = R
    // v*P = (k - u)*G
    // v*e*G = (k - u)*G
    // v*e = k - u
    // u = k - v*e;
    let v = 23; // a random number
    let k_ff = FiniteField::new(k as i32, P).unwrap();
    let u = k_ff - k_ff.sibling(v) * k_ff.sibling(e as i32);

    (u.num as u32, v as u32, pk)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_g_on_curve() {
        let y = pow_mod(G.x, 3, P) + 7;
        assert_eq!(y, G.y);
    }

    #[test]
    fn t_demo() {
        let ec = EC::new(A, B, P).unwrap();

        let k = 42;
        let (u, v, pk) = demo(58391, 42);
        println!("{:?}", (u, v, pk));

        // uG+vP=kG
        let p1 = ec.rmul(G, u).unwrap();
        let p2 = ec.rmul(pk, v).unwrap();
        let p3 = ec.rmul(G, k).unwrap();
        assert_eq!(ec.add(p1, p2), Some(p3));
    }
}
