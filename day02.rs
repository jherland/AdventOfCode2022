use std::io;
use std::str::FromStr;

use anyhow::{anyhow, Error, Result};

#[derive(Debug, Clone, Copy)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    fn fight(&self, opponent: &RPS) -> Outcome {
        match (self, opponent) {
            (RPS::Rock, RPS::Rock) | (RPS::Paper, RPS::Paper) | (RPS::Scissors, RPS::Scissors) => {
                Outcome::Draw
            }
            (RPS::Rock, RPS::Paper) | (RPS::Paper, RPS::Scissors) | (RPS::Scissors, RPS::Rock) => {
                Outcome::Loss
            }
            (RPS::Rock, RPS::Scissors) | (RPS::Paper, RPS::Rock) | (RPS::Scissors, RPS::Paper) => {
                Outcome::Win
            }
        }
    }

    fn score(&self, opponent: &RPS) -> u32 {
        let mychoice = match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        };
        let outcome = match self.fight(opponent) {
            Outcome::Loss => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        };
        mychoice + outcome
    }

    fn choose(&self, outcome: &Outcome) -> RPS {
        match &outcome {
            Outcome::Win => match &self {
                RPS::Rock => RPS::Paper,
                RPS::Paper => RPS::Scissors,
                RPS::Scissors => RPS::Rock,
            },
            Outcome::Draw => *self,
            Outcome::Loss => match &self {
                RPS::Rock => RPS::Scissors,
                RPS::Paper => RPS::Rock,
                RPS::Scissors => RPS::Paper,
            },
        }
    }
}

impl FromStr for RPS {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(RPS::Rock),
            "B" | "Y" => Ok(RPS::Paper),
            "C" | "Z" => Ok(RPS::Scissors),
            _ => Err(anyhow!("Failed to parse RPS from {s:?}")),
        }
    }
}

#[derive(Debug)]
enum Outcome {
    Win,
    Draw,
    Loss,
}

impl FromStr for Outcome {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcome::Loss),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err(anyhow!("Failed to parse Outcome from {s:?}")),
        }
    }
}

fn main() {
    let lines: Vec<_> = io::stdin().lines().map(Result::unwrap).collect();
    let char_pairs: Vec<_> = lines
        .iter()
        .map(|line| line.split_once(" ").unwrap())
        .collect();

    println!(
        "Part 1: {}",
        char_pairs
            .iter()
            .map(|(a, b)| b.parse::<RPS>().unwrap().score(&a.parse().unwrap()))
            .sum::<u32>()
    );

    println!(
        "Part 2: {}",
        char_pairs
            .iter()
            .map(|(a, b)| {
                let opponent: RPS = a.parse().unwrap();
                let outcome: Outcome = b.parse().unwrap();
                opponent.choose(&outcome).score(&opponent)
            })
            .sum::<u32>()
    );
}
