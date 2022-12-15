use std::io;
use std::ops::Add;
use std::str::FromStr;

use anyhow::{Error, Result};
use text_io::scan;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialOrd, PartialEq)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn mdist(&self, other: Self) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

impl Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

#[derive(Debug)]
struct Sensor {
    pos: Pos,
    beacon: Pos,
}

impl FromStr for Sensor {
    type Err = Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (sx, sy, bx, by): (i32, i32, i32, i32);
        scan!(line.bytes() => "Sensor at x={}, y={}: closest beacon is at x={}, y={}", sx, sy, bx, by);
        Ok(Sensor {
            pos: Pos::new(sx, sy),
            beacon: Pos::new(bx, by),
        })
    }
}

impl Sensor {
    fn radius(&self) -> u32 {
        self.pos.mdist(self.beacon)
    }

    fn bounds(&self) -> [Pos; 4] {
        [
            self.pos + Pos::new(-(self.radius() as i32), 0),
            self.pos + Pos::new(0, -(self.radius() as i32)),
            self.pos + Pos::new(self.radius() as i32, 0),
            self.pos + Pos::new(0, self.radius() as i32),
        ]
    }

    fn within(&self, pos: Pos) -> bool {
        // pos is within self's radius
        self.pos.mdist(pos) <= self.radius()
    }

    fn x_run(&self, y: i32) -> Option<(i32, i32)> {
        let remains = (self.radius() as i32) - (self.pos.y.abs_diff(y) as i32);
        if remains >= 0 {
            Some((self.pos.x - remains, self.pos.x + remains))
        } else {
            None
        }
    }
}

fn first_uncovered_pos_at_row(sensors: &[Sensor], y: i32, x_range: (i32, i32)) -> Option<Pos> {
    let (mut x, x_max) = x_range;
    let mut x_runs: Vec<(i32, i32)> = sensors.iter().filter_map(|s| s.x_run(y)).collect();
    x_runs.sort();
    for (a, b) in x_runs {
        if a <= x && x <= b {
            x = b + 1;
        }
        if x >= x_max {
            return None;
        }
    }
    Some(Pos::new(x, y))
}

fn main() {
    let mut sensors: Vec<Sensor> = io::stdin()
        .lines()
        .map(Result::unwrap)
        .map(|line| line.parse().unwrap())
        .collect();

    // Sort sensors by decreasing radius, to eliminate the biggest chunks first.
    // sensors.sort_by_key(|s| -(s.radius() as i32));
    sensors.sort_by_key(|s| s.pos.x - s.radius() as i32);

    let min_x = sensors
        .iter()
        .flat_map(|s| s.bounds())
        .map(|p| p.x)
        .min()
        .unwrap();
    let max_x = sensors
        .iter()
        .flat_map(|s| s.bounds())
        .map(|p| p.x)
        .max()
        .unwrap();

    println!(
        "Part 1: {}",
        (min_x..=max_x)
            .map(|x| Pos::new(x, 2_000_000))
            .filter(|p| sensors.iter().any(|s| s.within(*p) && s.beacon != *p))
            .count()
    );

    for y in 0..=4_000_000 {
        if let Some(pos) = first_uncovered_pos_at_row(&sensors, y, (0, 4_000_000)) {
            println!("Part 2: {}", pos.x as u64 * 4_000_000u64 + pos.y as u64);
            return;
        }
    }
}
