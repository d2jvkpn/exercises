#![allow(dead_code)]

use std::time::Duration;

// 04
enum TrafficLight {
    Green,
    Red,
    Yellow,
}

trait LastFor {
    fn last_for(&self) -> Duration;
}

impl LastFor for TrafficLight {
    fn last_for(&self) -> Duration {
        match self {
            Self::Green => Duration::new(10, 0),
            Self::Red => Duration::new(7, 0),
            Self::Yellow => Duration::new(3, 0),
        }
    }
}

// 05
fn sum_of_u32_slice(slice: &[u32]) -> Option<u32> {
    let mut sum: u32 = 0;

    for v in slice {
        if u32::MAX - sum < *v {
            return None;
        }
        sum += *v;
    }

    Some(sum)
}

// 06
struct Circle {
    radius: f64,
}

struct Triangle(f64, f64, f64);

struct Square {
    side: f64,
}

pub trait Area {
    fn area(&self) -> f64;
}

impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powi(2) / 2.0
    }
}

impl Area for Triangle {
    fn area(&self) -> f64 {
        let s = (self.0 + self.1 + self.2) / 2.0;
        (s * (s - self.0) * (s - self.1) * (s - self.2)).powf(0.5)
    }
}

impl Area for Square {
    fn area(&self) -> f64 {
        self.side.powi(2)
    }
}

fn get_area<T: Area>(item: T) -> f64 {
    item.area()
}

// tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn t_traffic_light() {
        let color = TrafficLight::Red;
        assert_eq!(color.last_for(), Duration::new(7, 0));
    }

    #[test]
    fn t_sum_of_u32_slice() {
        let items = vec![1, 2, 3, 4, 5];
        assert_eq!(sum_of_u32_slice(&items), Some(15));

        assert_eq!(sum_of_u32_slice(&[u32::MAX - 10, 10, 1]), None);
    }

    #[test]
    fn t_get_area() {
        let circle = Circle { radius: 1.0 };
        assert_eq!(
            get_area(circle),
            std::f64::consts::PI * 1.0_f64.powi(2) / 2.0
        );

        let triangle = Triangle(3.0, 4.0, 5.0);
        assert_eq!(get_area(triangle), 3.0 * 4.0 / 2.0);

        let square = Square { side: 2.0 };
        assert_eq!(get_area(square), 2.0 * 2.0);
    }
}
