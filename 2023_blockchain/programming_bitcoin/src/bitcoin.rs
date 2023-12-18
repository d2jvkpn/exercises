#![allow(dead_code)]

// secp256k1
// y**2 = x**3 + 7
// 256 bits
pub const P: &str = "2**256 - 2**32 - 977";

// 256 bits, x, y
pub const G: (&str, &str) = (
    "0x79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
    "0x483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8",
);

/*
assert_eq!(G.y**2 % P, (G.x**3 + 7) % P);

P: Point(x, y), public key;
e: u256, private key;
G: Point(x, y), const;

P = e * G;
*/

// ECDSA: Elliptic Curve Digital Signature Algorithm
