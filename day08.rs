use std::cmp::min;
use std::io;

#[derive(Clone, Copy, Debug)]
struct Point {
    y: usize,
    x: usize,
}

struct TreeMap {
    map: Vec<Vec<u8>>,
    height: usize,
    width: usize,
}

impl TreeMap {
    fn parse<I>(lines: I) -> Self
    where
        I: Iterator<Item = String>,
    {
        let mut width = None;
        let mut map = Vec::new();
        for line in lines {
            let mut row = Vec::new();
            for byte in line.as_bytes() {
                row.push(byte - b'0');
            }
            match width {
                None => width = Some(row.len()),
                Some(n) => assert!(n == row.len()),
            }
            map.push(row);
        }
        let height = map.len();
        let width = width.unwrap();
        Self { map, height, width }
    }

    fn contains(&self, p: Point) -> bool {
        p.y < self.height && p.x < self.width
    }

    fn points(&self) -> impl Iterator<Item = Point> {
        let width = self.width;
        (0..self.height).flat_map(move |y| (0..width).map(move |x| Point { y, x }))
    }

    fn get(&self, p: Point) -> u8 {
        assert!(self.contains(p));
        self.map[p.y][p.x]
    }

    fn to_edge(&self, p: Point) -> [Vec<Point>; 4] {
        let up = (0..p.y).rev().map(|y| Point { y, x: p.x }).collect();
        let down = ((p.y + 1)..self.height)
            .map(|y| Point { y, x: p.x })
            .collect();
        let left = (0..p.x).rev().map(|x| Point { y: p.y, x }).collect();
        let right = ((p.x + 1)..self.width)
            .map(|x| Point { y: p.y, x })
            .collect();
        [up, down, left, right]
    }

    fn tree_is_visible_from_edge(&self, p: Point) -> bool {
        assert!(self.contains(p));
        let tree = self.get(p);
        self.to_edge(p)
            .iter()
            .any(|dir| dir.iter().all(|&q| self.get(q) < tree))
    }

    fn num_trees_visible_from_tree(&self, p: Point) -> [usize; 4] {
        assert!(self.contains(p));
        let tree = self.get(p);
        self.to_edge(p).map(|v| {
            min(
                v.len(),
                v.iter().take_while(|&&q| self.get(q) < tree).count() + 1,
            )
        })
    }

    fn scenic_score(&self, p: Point) -> usize {
        self.num_trees_visible_from_tree(p).iter().product()
    }
}

pub fn main() {
    let map = TreeMap::parse(io::stdin().lines().map(Result::unwrap));
    println!(
        "Part 1: {}",
        map.points()
            .filter(|&p| map.tree_is_visible_from_edge(p))
            .count()
    );
    println!(
        "Part 2: {}",
        map.points().map(|p| map.scenic_score(p)).max().unwrap()
    );
}
