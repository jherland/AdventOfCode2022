use std::cmp::Ordering;
use std::io;
use std::str::FromStr;

use anyhow::{Error, Ok, Result};
use json::JsonValue;

#[derive(Debug, Eq)]
enum Packet {
    Int(u64),
    List(Vec<Packet>),
}

use Packet::*;

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Int(a), Int(b)) => a.cmp(b),
            (Int(a), b) => List(vec![Int(*a)]).cmp(b),
            (a, Int(b)) => a.cmp(&List(vec![Int(*b)])),
            (List(a), List(b)) => a.cmp(b),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Int(a), Int(b)) => a.partial_cmp(b),
            (Int(a), b) => List(vec![Int(*a)]).partial_cmp(b),
            (a, Int(b)) => a.partial_cmp(&List(vec![Int(*b)])),
            (List(a), List(b)) => a.partial_cmp(b),
        }
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Int(a), Int(b)) => a.eq(b),
            (Int(a), b) => List(vec![Int(*a)]).eq(b),
            (a, Int(b)) => a.eq(&List(vec![Int(*b)])),
            (List(a), List(b)) => a.eq(b),
        }
    }
}

fn packet_from_json(value: &JsonValue) -> Packet {
    use json::JsonValue::*;
    match value {
        Array(objs) => List(objs.iter().map(packet_from_json).collect()),
        Number(n) => Int(n.as_fixed_point_u64(0).unwrap()),
        _ => unreachable!("How to parse {value:?}!?"),
    }
}

impl FromStr for Packet {
    type Err = Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        Ok(packet_from_json(&json::parse(line)?))
    }
}

fn main() {
    let lines: Vec<_> = io::stdin().lines().map(Result::unwrap).collect();
    let pairs: Vec<(Packet, Packet)> = lines
        .split(String::is_empty)
        .map(|lines| match lines {
            [left, right] => (left.parse().unwrap(), right.parse().unwrap()),
            _ => unreachable!("Failed to find packet pair: {lines:?}"),
        })
        .collect();

    println!(
        "Part 1: {}",
        pairs
            .iter()
            .enumerate()
            .map(|(i, (lhs, rhs))| if lhs < rhs { i + 1 } else { 0 })
            .sum::<usize>()
    );

    let mut packets: Vec<_> = pairs.iter().flat_map(|(lhs, rhs)| [lhs, rhs]).collect();
    let div2: Packet = "[[2]]".parse().unwrap();
    let div6: Packet = "[[6]]".parse().unwrap();
    packets.push(&div2);
    packets.push(&div6);
    packets.sort();
    let i2 = packets.iter().position(|&p| p == &div2).unwrap();
    let i6 = packets.iter().position(|&p| p == &div6).unwrap();
    println!("Part 2: {}", (i2 + 1) * (i6 + 1));
}
