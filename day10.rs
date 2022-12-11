use std::cmp::Ordering;
use std::io;
use std::str::FromStr;

use anyhow::{Error, Ok, Result};

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i32),
}

use Instruction::*;

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        if let Some((d, n)) = line.split_once(' ') {
            assert!(d == "addx");
            Ok(Addx(n.parse()?))
        } else {
            assert!(line == "noop");
            Ok(Noop)
        }
    }
}

fn extract_x_at_cycle(execution: &Vec<(u32, i32)>, n: u32) -> i32 {
    let mut last = 0;
    for &(t, x) in execution {
        match n.cmp(&t) {
            Ordering::Equal => return x,
            Ordering::Less => return last,
            Ordering::Greater => (),
        };
        last = x;
    }
    unreachable!("Gone too far!");
}

pub fn main() {
    let instructions: Vec<Instruction> = io::stdin()
        .lines()
        .map(Result::unwrap)
        .map(|line| line.parse().unwrap())
        .collect();

    let mut execution: Vec<(u32, i32)> = vec![(1, 1)]; // during cycle #1, x is 1
    for instr in instructions {
        let (t, x) = execution[execution.len() - 1];
        execution.push(match instr {
            Noop => (t + 1, x),
            Addx(n) => (t + 2, x + n),
        });
    }
    println!(
        "Part 1: {}",
        [20u32, 60, 100, 140, 180, 220]
            .map(|t| t as i32 * extract_x_at_cycle(&execution, t))
            .iter()
            .sum::<i32>()
    );

    println!("Part 2:");
    let mut i = 0;
    for t in 0..(40 * 6) {
        if t > execution[i].0 as usize {
            i += 1;
        }
        let x = execution[i].1;
        let on = (x).abs_diff((t % 40) as i32) <= 1;
        print!("{}", if on { "#" } else { " " });
        if (t + 1) % 40 == 0 {
            println!();
        }
    }
}
