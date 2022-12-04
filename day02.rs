use std::io;
use std::str::FromStr;

use anyhow::{anyhow, Error, Result};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

use Move::*;

impl FromStr for Move {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Rock),
            "B" | "Y" => Ok(Paper),
            "C" | "Z" => Ok(Scissors),
            _ => Err(anyhow!("Failed to parse RPS from {s:?}")),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Outcome {
    Win,
    Draw,
    Loss,
}

use Outcome::*;

impl FromStr for Outcome {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Loss),
            "Y" => Ok(Draw),
            "Z" => Ok(Win),
            _ => Err(anyhow!("Failed to parse Outcome from {s:?}")),
        }
    }
}

struct Fight(Move, Move, Outcome);

const VALID_FIGHTS: [Fight; 9] = [
    // (opponent move, my move, outcome)
    Fight(Rock, Paper, Win),
    Fight(Paper, Scissors, Win),
    Fight(Scissors, Rock, Win),
    Fight(Rock, Rock, Draw),
    Fight(Paper, Paper, Draw),
    Fight(Scissors, Scissors, Draw),
    Fight(Rock, Scissors, Loss),
    Fight(Paper, Rock, Loss),
    Fight(Scissors, Paper, Loss),
];

impl Fight {
    fn parse_move_vs_move(line: &str) -> &'static Fight {
        let (a, b) = line.split_once(' ').unwrap();
        VALID_FIGHTS
            .iter()
            .find(|f| f.0 == a.parse().unwrap() && f.1 == b.parse().unwrap())
            .unwrap()
    }

    fn parse_move_and_outcome(line: &str) -> &'static Fight {
        let (a, c) = line.split_once(' ').unwrap();
        VALID_FIGHTS
            .iter()
            .find(|f| f.0 == a.parse().unwrap() && f.2 == c.parse().unwrap())
            .unwrap()
    }

    fn score(&self) -> u32 {
        (match self.1 {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }) + (match self.2 {
            Loss => 0,
            Draw => 3,
            Win => 6,
        })
    }
}

fn main() {
    let lines: Vec<_> = io::stdin().lines().map(Result::unwrap).collect();

    println!(
        "Part 1: {}",
        lines
            .iter()
            .map(|line| Fight::parse_move_vs_move(line).score())
            .sum::<u32>()
    );

    println!(
        "Part 2: {}",
        lines
            .iter()
            .map(|line| Fight::parse_move_and_outcome(line).score())
            .sum::<u32>()
    );
}
