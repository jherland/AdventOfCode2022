use std::io;

use itertools::Itertools;

fn main() {
    let lines: Vec<_> = io::stdin().lines().map(Result::unwrap).collect();
    let paragraphs: Vec<_> = lines.split(|l| l == "").collect();
    let elves: Vec<_> = paragraphs
        .iter()
        .map(|&lines| lines.iter().map(|l| l.parse::<u32>().unwrap()).sum())
        .sorted()
        .rev()
        .collect();
    println!("Part 1: {}", elves[0]);
    println!("Part 2: {}", elves[..3].iter().sum::<u32>());
}
