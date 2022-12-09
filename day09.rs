use std::collections::HashSet;
use std::io;
use std::ops::Add;
use std::str::FromStr;

use anyhow::{anyhow, Error, Result};

#[derive(Clone, Copy, Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

use Dir::*;

impl FromStr for Dir {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Up),
            "D" => Ok(Down),
            "L" => Ok(Left),
            "R" => Ok(Right),
            _ => Err(anyhow!("Failed to parse direction from {s:?}")),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Move {
    dir: Dir,
    dist: usize,
}

impl FromStr for Move {
    type Err = Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (d, n) = line.split_once(' ').ok_or(anyhow!("No space"))?;
        Ok(Move {
            dir: d.parse()?,
            dist: n.parse()?,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Pos {
    y: i32,
    x: i32,
}

impl Pos {
    fn walk(self, dir: Dir) -> Self {
        match dir {
            Up => self + Pos { y: -1, x: 0 },
            Down => self + Pos { y: 1, x: 0 },
            Left => self + Pos { y: 0, x: -1 },
            Right => self + Pos { y: 0, x: 1 },
        }
    }

    fn adjacent(self, other: Self) -> bool {
        self.y.abs_diff(other.y) <= 1 && self.x.abs_diff(other.x) <= 1
    }

    fn follow(self, other: Self) -> Self {
        if self.adjacent(other) {
            return self;
        }
        let dy = (other.y - self.y).clamp(-1, 1);
        let dx = (other.x - self.x).clamp(-1, 1);
        self + Pos { y: dy, x: dx }
    }
}

impl Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            y: self.y + rhs.y,
            x: self.x + rhs.x,
        }
    }
}

struct Rope {
    knots: Vec<Pos>,
}

impl Rope {
    fn new(num_knots: usize) -> Self {
        Rope {
            knots: vec![Pos { y: 0, x: 0 }; num_knots],
        }
    }

    fn tail(&self) -> Pos {
        self.knots[self.knots.len() - 1]
    }

    fn drag(&mut self, dir: Dir) {
        self.knots[0] = self.knots[0].walk(dir);
        for i in 1..self.knots.len() {
            self.knots[i] = self.knots[i].follow(self.knots[i - 1]);
        }
    }
}

pub fn main() {
    let moves: Vec<Move> = io::stdin()
        .lines()
        .map(Result::unwrap)
        .map(|line| line.parse().unwrap())
        .collect();

    let mut rope = Rope::new(2);
    let mut tail_history: HashSet<Pos> = HashSet::new();
    for dir in moves.iter().flat_map(|m| (0..m.dist).map(|_| m.dir)) {
        rope.drag(dir);
        tail_history.insert(rope.tail());
    }
    println!("Part 1: {}", tail_history.len());

    let mut rope = Rope::new(10);
    tail_history = HashSet::new();
    for dir in moves.iter().flat_map(|m| (0..m.dist).map(|_| m.dir)) {
        rope.drag(dir);
        tail_history.insert(rope.tail());
    }
    println!("Part 2: {}", tail_history.len());
}
