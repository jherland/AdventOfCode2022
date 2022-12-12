use std::cmp::min;
use std::collections::{HashMap, HashSet};
use std::io;
use std::ops::Add;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Pos {
    y: i32,
    x: i32,
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

impl Pos {
    fn up(&self) -> Self {
        *self + Pos { y: -1, x: 0 }
    }

    fn down(&self) -> Self {
        *self + Pos { y: 1, x: 0 }
    }

    fn left(&self) -> Self {
        *self + Pos { y: 0, x: -1 }
    }

    fn right(&self) -> Self {
        *self + Pos { y: 0, x: 1 }
    }
}

type Height = u8;

#[derive(Debug)]
struct Map(HashMap<Pos, Height>);

impl Map {
    fn get(&self, p: Pos) -> Option<Height> {
        self.0.get(&p).copied()
    }

    fn adjacents(&self, p: Pos) -> HashSet<Pos> {
        let mut ret = HashSet::new();
        let h = self.get(p).unwrap();
        for nbor in [p.up(), p.down(), p.left(), p.right()] {
            if let Some(nh) = self.get(nbor) {
                if nh <= h + 1 {
                    ret.insert(nbor);
                }
            }
        }
        ret
    }

    fn shortest_path<F>(&self, start: Pos, end: F) -> u32
    where
        F: FnOnce(Pos) -> bool + Copy,
    {
        // Dijkstra's algorithm!
        let mut unvisited: HashSet<Pos> = self.0.keys().cloned().collect();
        let mut dist: HashMap<Pos, u32> = HashMap::new();
        dist.insert(start, 0);

        loop {
            let candidates: HashSet<Pos> = unvisited
                .intersection(&dist.keys().copied().collect())
                .copied()
                .collect();
            let current = *candidates
                .iter()
                .min_by_key(|&p| dist.get(p).unwrap())
                .unwrap();
            let cur_dist = *dist.get(&current).unwrap();
            if end(current) {
                return cur_dist;
            }
            for nbor in self
                .adjacents(current)
                .intersection(&unvisited)
                .copied()
                .collect::<HashSet<Pos>>()
            {
                let d = cur_dist + 1;
                let old_d = *dist.get(&nbor).unwrap_or(&d);
                dist.insert(nbor, min(d, old_d));
            }
            unvisited.remove(&current);
        }
    }

    fn flipped(&self) -> Map {
        Map(self.0.iter().map(|(p, h)| (*p, h.abs_diff(25))).collect())
    }
}

fn parse(lines: &[String]) -> (Map, Pos, Pos) {
    let mut heights = HashMap::new();
    let mut start = Pos { y: 0, x: 0 };
    let mut end = Pos { y: 0, x: 0 };
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.as_bytes().iter().enumerate() {
            let p = Pos {
                y: y as i32,
                x: x as i32,
            };
            let h = match c {
                b'S' => {
                    start = p;
                    0
                }
                b'E' => {
                    end = p;
                    25
                }
                _ => c - b'a',
            };
            heights.insert(p, h);
        }
    }
    (Map(heights), start, end)
}

fn main() {
    let lines: Vec<_> = io::stdin().lines().map(Result::unwrap).collect();
    let (map, start, end) = parse(&lines);

    println!("Part 1: {}", map.shortest_path(start, |p| p == end));

    println!(
        "Part 2: {}",
        map.flipped()
            .shortest_path(end, |p| map.get(p).unwrap() == 0)
    );
}
