use std::collections::VecDeque;
// use std::cmp::Ordering;
use std::io;
// use std::str::FromStr;

// use anyhow::{Error, Ok, Result};

type Item = usize;

#[derive(Debug)]
enum Operation {
    Add(Item),
    Mult(Item),
    Square,
}

use Operation::*;

impl Operation {
    fn null() -> Self {
        Add(0)
    }

    fn parse(s: &str) -> Self {
        let words: Vec<_> = s.split(' ').collect();
        assert!(words.len() == 3);
        assert!(words[0] == "old");
        match (words[1], words[2]) {
            ("+", num) => Add(num.parse().unwrap()),
            ("*", "old") => Square,
            ("*", num) => Mult(num.parse().unwrap()),
            _ => unreachable!("Invalid operation: {s:?}")
        }
    }

    fn call(&self, old: Item) -> Item {
        match self {
            Add(n) => old + n,
            Mult(n) => old * n,
            Square => old * old,
        }
    }
}

#[derive(Debug)]
struct DivisibleBy(Item);

impl DivisibleBy {
    fn test(&self, item: Item) -> bool {
        item % self.0 == 0
    }
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<Item>,
    operation: Operation,
    test: DivisibleBy,
    if_true: usize,
    if_false: usize,
    activity: usize,
}

impl Monkey {
    // fn parse<I>(lines: I) -> Self
    // where
    //     I: Iterator<Item = &str>,
    fn parse(lines: &[String]) -> Self
    {
        let mut items: VecDeque<Item> = VecDeque::new();
        let mut operation = Operation::null();
        let mut test = DivisibleBy(1);
        let mut if_true = 0;
        let mut if_false = 0;
        for line in lines {
            if line.starts_with("Monkey ") {
                continue;
            } else if let Some(end) = line.strip_prefix("  Starting items: ") {
                items = end.split(',').map(|s| s.trim().parse().unwrap()).collect();
            } else if let Some(end) = line.strip_prefix("  Operation: new = ") {
                operation = Operation::parse(end);
            } else if let Some(end) = line.strip_prefix("  Test: divisible by ") {
                test = DivisibleBy(end.parse().unwrap());
            } else if let Some(end) = line.strip_prefix("    If true: throw to monkey ") {
                if_true = end.parse().unwrap();
            } else if let Some(end) = line.strip_prefix("    If false: throw to monkey ") {
                if_false = end.parse().unwrap();
            } else {
                unreachable!("Cannot parse monkey w/line: {line:?}");
            }
        }
        Monkey { items, operation, test, if_true, if_false, activity: 0 }
    }

    fn do_round(&mut self) -> Vec<(usize, Item)> {
        let mut throws = Vec::new();
        while let Some(item) = self.items.pop_front() {
            let post_op = self.operation.call(item) / 3;
            let target = if self.test.test(post_op) { self.if_true } else {self.if_false };
            throws.push((target, post_op));
            self.activity += 1;
        }
        throws
    }
}

fn monkey_business(monkeys: &Vec<Monkey>) -> usize {
    assert!(monkeys.len() >= 2);
    let mut activities: Vec<_> = monkeys.iter().map(|m| m.activity).collect();
    activities.sort();
    activities.iter().rev().take(2).product()
}

fn main() {
    let lines: Vec<_> = io::stdin().lines().map(Result::unwrap).collect();
    let paragraphs: Vec<_> = lines.split(String::is_empty).collect();
    let mut monkeys: Vec<_> = paragraphs.iter().map(|&lines| Monkey::parse(lines)).collect();

    for round in 1..=20 {
        for i in 0..monkeys.len() {
            let throws = monkeys[i].do_round();
            dbg!(&throws);
            for (target, item) in throws {
                monkeys[target].items.push_back(item);
            }
        }
        println!("After round {round}:");
        dbg!(&monkeys);
    }
    println!("Part 1: {}", monkey_business(&monkeys));
}
