use std::cmp::{max, min};
use std::collections::HashSet;
use std::io;
use std::ops::Add;
use std::str::FromStr;

use anyhow::{anyhow, Error, Result};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialOrd, PartialEq)]
struct Pos {
    x: u32,
    y: u32,
}

impl Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl FromStr for Pos {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(',') {
            Some((x, y)) => Ok(Self {
                x: x.parse()?,
                y: y.parse()?,
            }),
            _ => Err(anyhow!("Failed to parse {s:?}")),
        }
    }
}

impl Pos {
    fn line_between(&self, other: Pos) -> impl Iterator<Item = Pos> + '_ {
        assert!(self.x == other.x || self.y == other.y);
        assert!(self < &other);
        (self.x..=other.x).flat_map(move |x| (self.y..=other.y).map(move |y| Pos { x, y }))
    }

    fn down(&self) -> Pos {
        Pos {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn down_left(&self) -> Pos {
        Pos {
            x: self.x - 1,
            y: self.y + 1,
        }
    }

    fn down_right(&self) -> Pos {
        Pos {
            x: self.x + 1,
            y: self.y + 1,
        }
    }
}

#[derive(Debug)]
struct Cave {
    rocks: HashSet<Pos>,
    start: Pos,
    sand: HashSet<Pos>,
}

impl Cave {
    fn parse(lines: &[String], start: Pos) -> Self {
        let mut rocks = HashSet::new();
        for line in lines {
            let corners: Vec<_> = line
                .split(" -> ")
                .map(|s| s.parse::<Pos>().unwrap())
                .collect();
            for rock in corners.windows(2).flat_map(|ends| match ends {
                [a, b] => min(a, b).line_between(*max(a, b)),
                _ => unreachable!(".windows() failure!"),
            }) {
                rocks.insert(rock);
            }
        }
        Self {
            rocks,
            start,
            sand: HashSet::new(),
        }
    }

    fn bottom(&self) -> u32 {
        self.rocks.iter().map(|pos| pos.y).max().unwrap()
    }

    fn _render(&self) {
        let min_x = self.rocks.iter().map(|pos| pos.x).min().unwrap();
        let max_x = self.rocks.iter().map(|pos| pos.x).max().unwrap();
        let max_y = self.bottom();
        for y in 0..=max_y {
            for x in min_x..=max_x {
                let p = Pos { x, y };
                let c = if self.rocks.contains(&p) {
                    '#'
                } else if self.sand.contains(&p) {
                    'o'
                } else if p == self.start {
                    '+'
                } else {
                    '.'
                };
                print!("{}", c);
            }
            println!();
        }
    }

    fn occupied(&self, pos: Pos) -> bool {
        self.rocks.contains(&pos) || self.sand.contains(&pos)
    }

    fn fall(&self, pos: Pos) -> Pos {
        for next in [pos.down(), pos.down_left(), pos.down_right()] {
            if !self.occupied(next) {
                return next;
            }
        }
        pos
    }

    fn fall_until_rest(&self) -> Option<Pos> {
        let mut cur = self.start;
        loop {
            let next = self.fall(cur);
            if next == cur {
                return Some(cur);
            } else if next.y > self.bottom() {
                return None;
            }
            cur = next;
        }
    }

    fn rest(&mut self, pos: Pos) {
        self.sand.insert(pos);
    }
}

fn main() {
    let lines: Vec<_> = io::stdin().lines().map(Result::unwrap).collect();
    let mut cave = Cave::parse(&lines, Pos { x: 500, y: 0 });

    while let Some(pos) = cave.fall_until_rest() {
        cave.rest(pos)
    }
    // cave._render();
    println!("Part 1: {}", cave.sand.len());

    let floor_y = cave.bottom() + 2;
    let floor_l = Pos {
        x: cave.start.x - floor_y,
        y: floor_y,
    };
    let floor_r = Pos {
        x: cave.start.x + floor_y,
        y: floor_y,
    };
    floor_l.line_between(floor_r).for_each(|pos| {
        cave.rocks.insert(pos);
    });
    while let Some(pos) = cave.fall_until_rest() {
        cave.rest(pos);
        if pos == cave.start {
            break;
        }
    }
    // cave._render();
    println!("Part 2: {}", cave.sand.len());
}
