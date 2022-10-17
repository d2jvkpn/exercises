pub fn is_power_of_three(n: i32) -> bool {
    let mut v = n;

    if v < 0 {
        return false;
    }

    while v > 0 {
        if v == 1 {
            return true;
        }
        if v % 3 != 0 {
            return false;
        }
        v /= 3;
    }

    false
}

fn roman_to_int(s: String) -> i32 {
    use std::collections::HashMap;

    let mp = HashMap::from([
        ("I", 1),
        ("IV", 4),
        ("V", 5),
        ("IX", 9),
        ("X", 10),
        ("XL", 40),
        ("L", 50),
        ("XC", 90),
        ("C", 100),
        ("CD", 400),
        ("D", 500),
        ("CM", 900),
        ("M", 1000),
    ]);

    let len = s.len();
    let (mut i, mut result) = (0, 0);

    while i < len {
        if i == len - 1 {
            result += *mp.get(&s[i..i + 1]).unwrap_or(&0);
            break;
        }

        result += if let Some(v) = mp.get(&s[i..i + 2]) {
            i += 1;
            *v
        } else {
            *mp.get(&s[i..i + 1]).unwrap_or(&0)
        };
        i += 1;
    }

    result
}

pub fn count_primes(n: i32) -> i32 {
    if n <= 1 {
        return 0;
    }

    let s = n as usize;
    let mut is_prime = vec![true; s];
    let mut i: usize = 2;

    while i * i < s {
        // println!("~~~ {:?}", is_prime);
        if is_prime[i] {
            for j in (i * i..s).step_by(i) {
                is_prime[j] = false;
            }
        }

        i += 1;
    }

    is_prime[2..].iter().filter(|&v| *v).count() as i32
}

fn main() {
    println!(">>> is_power_of_three: {}", is_power_of_three(27));
    println!(">>> roman_to_int: {}", roman_to_int("MCMXCIV".to_string()));
    println!(">>> count_primes: {}", count_primes(10));
    println!(">>> count_primes: {}", count_primes(0));
    println!(">>> count_primes: {}", count_primes(2));
}
