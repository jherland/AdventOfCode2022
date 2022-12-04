use std::io;

use itertools::Itertools;

fn common<T: std::cmp::PartialEq + Copy>(a: &[T], b: &[T]) -> Vec<T> {
    a.iter().copied().filter(|c| b.iter().contains(c)).collect()
}

fn first_common<T: std::cmp::PartialEq + Copy>(a: &[T], b: &[T]) -> Option<T> {
    common(a, b).get(0).copied()
}

fn score(c: u8) -> u32 {
    match c {
        b'a'..=b'z' => (c - b'a') as u32 + 1,
        b'A'..=b'Z' => (c - b'A') as u32 + 27,
        _ => unreachable!("Unrecognized letter {c:?}!"),
    }
}

fn halves<T>(items: &[T]) -> (&[T], &[T]) {
    assert!(items.len() % 2 == 0);
    items.split_at(items.len() / 2)
}

fn main() {
    let lines: Vec<_> = io::stdin().lines().map(Result::unwrap).collect();

    println!(
        "Part 1: {}",
        lines
            .iter()
            .map(|l| halves(l.as_bytes()))
            .map(|(a, b)| score(first_common(a, b).unwrap()))
            .sum::<u32>()
    );

    println!(
        "Part 2: {}",
        lines
            .iter()
            .tuples()
            .map(|(a, b, c)| {
                score(first_common(a.as_bytes(), &common(b.as_bytes(), c.as_bytes())).unwrap())
            })
            .sum::<u32>()
    );
}
