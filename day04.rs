use std::io;

use tuple_map::*;

#[derive(Debug)]
struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn from(t: (u32, u32)) -> Range {
        assert!(t.0 <= t.1);
        Range {
            start: t.0,
            end: t.1 + 1,
        }
    }

    fn contains(&self, other: &Range) -> bool {
        other.start >= self.start && other.end <= self.end
    }

    fn overlaps(&self, other: &Range) -> bool {
        if other.start < self.start {
            self.start < other.end
        } else {
            other.start < self.end
        }
    }
}

fn main() {
    let input: Vec<_> = io::stdin()
        .lines()
        .map(Result::unwrap)
        .map(|l| {
            l.split_once(',')
                .unwrap()
                .map(|s| {
                    s.split_once('-')
                        .unwrap()
                        .map(|s| s.parse::<u32>().unwrap())
                })
                .map(Range::from)
        })
        .collect();

    println!(
        "Part 1: {}",
        input
            .iter()
            .filter(|(a, b)| a.contains(b) || b.contains(a))
            .count()
    );

    println!(
        "Part 2: {}",
        input.iter().filter(|(a, b)| a.overlaps(b)).count()
    );
}
