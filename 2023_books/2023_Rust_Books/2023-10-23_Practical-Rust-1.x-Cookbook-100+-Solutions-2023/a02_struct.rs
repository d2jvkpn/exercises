#![allow(dead_code)]

use std::default::Default;

fn main() {
    let mut points: [Point; 5] = [Point { x: 0, y: 0 }; 5];
    for i in 0..5 {
        let val = i as i32;
        points[i] = Point { x: val, y: val * 2 };
    }

    let points: [Point; 5] = Default::default();
    println!("==> points: {:?}", points);

    let mut point = Point { x: 0, y: 0 };
    move_point(&mut point, Direction::North);
    println!("==> Moved to ({}, {})", point.x, point.y);
}

#[derive(Clone, Copy, Debug, Default)]
struct Point {
    x: i32,
    y: i32,
}

enum Direction {
    North,
    South,
    East,
    West,
}

fn move_point(point: &mut Point, dir: Direction) {
    match dir {
        Direction::North => point.y += 1,
        Direction::South => point.y -= 1,
        Direction::East => point.x += 1,
        Direction::West => point.x -= 1,
    }
}
