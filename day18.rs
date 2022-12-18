use std::cmp::{max, min};
use std::collections::BTreeSet;
use std::collections::HashSet;
use std::io;
use std::ops::Add;
use std::str::FromStr;

use anyhow::{anyhow, Error, Result};

type Coord = i32;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialOrd, PartialEq)]
struct Pos {
    x: Coord, // -left, +right
    y: Coord, // -below, +above
    z: Coord, // -behind, +infront
}

impl Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl FromStr for Pos {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(',') {
            Some((x, yz)) => match yz.split_once(',') {
                Some((y, z)) => Ok(Self {
                    x: x.parse()?,
                    y: y.parse()?,
                    z: z.parse()?,
                }),
                _ => Err(anyhow!("Failed to parse y/z from {s:?}")),
            },
            _ => Err(anyhow!("Failed to parse x/yz from {s:?}")),
        }
    }
}

impl Pos {
    fn new(x: Coord, y: Coord, z: Coord) -> Self {
        Self { x, y, z }
    }

    fn adjacents(self) -> [Self; 6] {
        [
            self + Pos::new(-1, 0, 0), // left
            self + Pos::new(1, 0, 0),  // right
            self + Pos::new(0, -1, 0), // below
            self + Pos::new(0, 1, 0),  // above
            self + Pos::new(0, 0, -1), // behind
            self + Pos::new(0, 0, 1),  // infront
        ]
    }
}

#[derive(Clone, Copy, Debug)]
struct BBox {
    min: Pos, // inclusive
    max: Pos, // exclusive
}

impl BBox {
    fn from(mut points: impl Iterator<Item = Pos>) -> Option<Self> {
        if let Some(Pos { x, y, z }) = points.next() {
            let (mut min_x, mut min_y, mut min_z) = (x, y, z);
            let (mut max_x, mut max_y, mut max_z) = (x, y, z);
            for Pos { x, y, z } in points {
                min_x = min(min_x, x);
                min_y = min(min_y, y);
                min_z = min(min_z, z);
                max_x = max(max_x, x);
                max_y = max(max_y, y);
                max_z = max(max_z, z);
            }
            Some(Self {
                min: Pos::new(min_x, min_y, min_z),
                max: Pos::new(max_x + 1, max_y + 1, max_z + 1),
            })
        } else {
            None
        }
    }

    fn _volume(&self) -> u32 {
        ((self.max.x - self.min.x) * (self.max.y - self.min.y) * (self.max.z - self.min.z)) as u32
    }

    fn contains(&self, p: &Pos) -> bool {
        self.min.x <= p.x
            && p.x < self.max.x
            && self.min.y <= p.y
            && p.y < self.max.y
            && self.min.z <= p.z
            && p.z < self.max.z
    }
}

fn spread(bbox: &BBox, start: Pos, barriers: &HashSet<Pos>) -> HashSet<Pos> {
    let mut active = BTreeSet::from([start]); // HashSet has no (efficient) .pop()
    let mut settled = HashSet::new();
    while let Some(cur) = active.iter().next().copied() {
        active.remove(&cur); // BTreeSet.pop_first() is still experimental, *grumble*
        settled.insert(cur);
        for next in cur.adjacents() {
            if bbox.contains(&next) && !settled.contains(&next) && !barriers.contains(&next) {
                active.insert(next);
            }
        }
    }
    settled
}

fn main() {
    let cubes: HashSet<Pos> = io::stdin()
        .lines()
        .map(Result::unwrap)
        .map(|s| s.parse())
        .map(Result::unwrap)
        .collect();

    let adjacents: Vec<Pos> = cubes
        .iter()
        .flat_map(|&p| p.adjacents().into_iter())
        .collect();
    let exposed_surface: Vec<Pos> = adjacents
        .iter()
        .filter(|&pos| !cubes.contains(pos))
        .copied()
        .collect();
    println!("Part 1: {}", exposed_surface.len());

    let bbox = BBox::from(exposed_surface.clone().into_iter()).unwrap();
    let steam = spread(&bbox, bbox.min, &cubes);
    println!(
        "Part 2: {}",
        exposed_surface.iter().filter(|p| steam.contains(p)).count()
    );
}
