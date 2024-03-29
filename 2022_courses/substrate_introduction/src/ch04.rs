#![allow(dead_code)]

use std::time::Duration;

//// 04
enum TrafficLight {
    Green(Duration),
    Red(Duration),
    Yellow(Duration),
}

trait LastFor {
    fn last_for(&self) -> Duration;
}

impl LastFor for TrafficLight {
    fn last_for(&self) -> Duration {
        //        match self {
        //            Self::Green => Duration::new(10, 0),
        //            Self::Red => Duration::new(7, 0),
        //            Self::Yellow => Duration::new(3, 0),
        //        }
        match self {
            Self::Green(v) => *v,
            Self::Red(v) => *v,
            Self::Yellow(v) => *v,
        }
    }
}

//// 05
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

fn sum_of_u32_slice_v2(slice: &[u32]) -> Option<u32> {
    let mut sum: u32 = 0;

    for v in slice {
        sum = sum.checked_add(*v)?;
    }

    Some(sum)
}

//// 06
pub trait Shape {
    fn area(&self) -> f64;
}

// Circle
struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powi(2) / 2.0
    }
}

// Triangle
struct Triangle(f64, f64, f64);

impl Shape for Triangle {
    fn area(&self) -> f64 {
        let s = (self.0 + self.1 + self.2) / 2.0;
        (s * (s - self.0) * (s - self.1) * (s - self.2)).powf(0.5)
    }
}

// Square
struct Square {
    side: f64,
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side.powi(2)
    }
}

// Rectangle
pub struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

//
fn get_area<T: Shape>(item: T) -> f64 {
    item.area()
}

// tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn t_traffic_light() {
        let color = TrafficLight::Red(Duration::new(7, 0));
        assert_eq!(color.last_for(), Duration::new(7, 0));
    }

    #[test]
    fn t_sum_of_u32_slice() {
        let items = vec![1, 2, 3, 4, 5];
        assert_eq!(sum_of_u32_slice(&items), Some(15));
        assert_eq!(sum_of_u32_slice(&[u32::MAX - 10, 10, 1]), None);

        assert_eq!(sum_of_u32_slice_v2(&items), Some(15));
        assert_eq!(sum_of_u32_slice_v2(&[u32::MAX - 10, 10, 1]), None);
    }

    #[test]
    fn t_get_area() {
        let circle = Circle { radius: 1.0 };
        assert_eq!(get_area(circle), std::f64::consts::PI * 1.0_f64.powi(2) / 2.0);

        let triangle = Triangle(3.0, 4.0, 5.0);
        assert_eq!(get_area(triangle), 3.0 * 4.0 / 2.0);

        let square = Square { side: 2.0 };
        assert_eq!(get_area(square), 2.0 * 2.0);

        let rectangle = Rectangle { width: 4.0, height: 2.0 };
        assert_eq!(get_area(rectangle), 4.0 * 2.0);
    }
}
