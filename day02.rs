use std::io;

#[derive(Debug, Clone, Copy)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum Outcome {
    Win,
    Draw,
    Loss,
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

fn main() {
    let mut chars: Vec<(u8, u8)> = Vec::new();
    for line in io::stdin().lines() {
        let line = line.expect("No line?");
        let bytes = line.trim().as_bytes();
        assert!(bytes.len() == 3);
        chars.push((bytes[0], bytes[2]));
    }

    // part 1
    let mut total = 0;
    for (first, second) in chars.iter() {
        let opponent = match first {
            b'A' => RPS::Rock,
            b'B' => RPS::Paper,
            b'C' => RPS::Scissors,
            _ => panic!("Parse error: {first:?}"),
        };
        let myself = match second {
            b'X' => RPS::Rock,
            b'Y' => RPS::Paper,
            b'Z' => RPS::Scissors,
            _ => panic!("Parse error: {second:?}"),
        };
        let score = myself.score(&opponent);
        total += score;
    }
    println!("{total}");

    // part 2
    let mut total = 0;
    for (first, second) in chars.iter() {
        let opponent = match first {
            b'A' => RPS::Rock,
            b'B' => RPS::Paper,
            b'C' => RPS::Scissors,
            _ => panic!("Parse error: {first:?}"),
        };
        let outcome = match second {
            b'X' => Outcome::Loss,
            b'Y' => Outcome::Draw,
            b'Z' => Outcome::Win,
            _ => panic!("Parse error: {second:?}"),
        };
        let myself = opponent.choose(&outcome);
        let score = myself.score(&opponent);
        total += score;
    }
    println!("{total}");
}
