use std::cmp::max;
use std::collections::{HashMap, HashSet, VecDeque};
use std::io;
use std::iter::Cycle;
use std::ops::Add;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialOrd, PartialEq)]
struct Pos {
    y: i32,
    x: i32,
}

impl Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            y: self.y + rhs.y,
            x: self.x + rhs.x,
        }
    }
}

impl Pos {
    fn new(y: i32, x: i32) -> Self {
        Self { y, x }
    }

    fn down() -> Self {
        Pos::new(-1, 0)
    }

    fn left() -> Self {
        Pos::new(0, -1)
    }

    fn right() -> Self {
        Pos::new(0, 1)
    }
}

enum Stone {
    HLine,
    Plus,
    Ell,
    VLine,
    Block,
}

use Stone::*;

impl Stone {
    fn next(&self) -> Self {
        match self {
            HLine => Plus,
            Plus => Ell,
            Ell => VLine,
            VLine => Block,
            Block => HLine,
        }
    }
}

#[derive(Clone, Debug)]
struct Rock(HashSet<Pos>);

impl Rock {
    fn new(model: &Stone) -> Self {
        // (0, 0) in each Rock is the lower left corner of its bounding box
        Self(
            match model {
                HLine => vec![(0, 0), (0, 1), (0, 2), (0, 3)],
                Plus => vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)],
                Ell => vec![(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)],
                VLine => vec![(0, 0), (1, 0), (2, 0), (3, 0)],
                Block => vec![(0, 0), (0, 1), (1, 0), (1, 1)],
            }
            .into_iter()
            .map(|(y, x)| Pos::new(y, x))
            .collect(),
        )
    }

    fn translate(&self, delta: Pos) -> Self {
        Self(self.0.iter().map(|p| *p + delta).collect())
    }
}

struct Chamber {
    width: u32,
    rocks: HashSet<Pos>,
    next_stone: Stone,
    falling: Option<Rock>,
    jets: VecDeque<Pos>,
    num_landed: u64,
}

impl Chamber {
    fn construct(width: u32, first_stone: Stone, jets: VecDeque<Pos>) -> Self {
        let mut ret = Self {
            width,
            rocks: HashSet::new(),
            next_stone: first_stone,
            falling: None,
            jets,
            num_landed: 0,
        };
        ret.next_rock();
        ret
    }

    fn top(&self) -> i32 {
        self.rocks.iter().map(|pos| pos.y + 1).max().unwrap_or_default()
    }

    fn top_w_falling(&self) -> i32 {
        max(
            self.top(),
            match &self.falling {
                Some(rock) => rock.0.iter().map(|pos| pos.y + 1).max().unwrap_or_default(),
                None => 0,
            },
        )
    }

    fn render_top(&self, lines: usize) -> String {
        let mut ret = String::new();
        let falling: HashSet<Pos> = match &self.falling {
            Some(rock) => rock.0.clone(),
            None => HashSet::new(),
        };
        for y in ((self.top_w_falling() - (lines as i32))..=self.top_w_falling()).rev() {
            ret.push_str("|");
            for x in 0..(self.width as i32) {
                let pos = Pos::new(y, x);
                if falling.contains(&pos) {
                    ret.push_str("@");
                } else if self.rocks.contains(&pos) {
                    ret.push_str("#");
                } else {
                    ret.push_str(".");
                }
            }
            ret.push_str("|\n");
        }
        ret.push_str("+");
        for _ in 0..self.width {
            ret.push_str("-");
        }
        ret.push_str("+\n");
        ret
    }

    fn render(&self) -> String {
        self.render_top(self.top_w_falling() as usize)
    }

    fn overlaps(&self, rock: &Rock) -> bool {
        rock.0.intersection(&self.rocks).count() > 0 ||
        rock.0.iter().any(|pos| pos.x < 0 || pos.x >= (self.width as i32) || pos.y < 0)
    }

    fn floats(&self, rock: &Rock) -> bool {
        !self.overlaps(&rock.translate(Pos::down()))
    }

    fn next_rock(&mut self) {
        assert!(self.falling.is_none());
        let start_pos = Pos::new(self.top() + 3, 2);
        let rock = Rock::new(&self.next_stone);
        self.next_stone = self.next_stone.next();
        self.falling = Some(rock.translate(start_pos));
    }

    fn move_rock(&mut self, delta: Pos) -> bool {
        if let Some(rock) = &self.falling {
            let moved = rock.translate(delta);
            if !self.overlaps(&moved) {
                self.falling = Some(moved);
                true
            } else {
                false
            }
        } else {
            unreachable!("No rock to move!");
        }
    }

    fn blow(&mut self, delta: Pos) -> bool {
        self.move_rock(delta)
    }

    fn fall(&mut self) -> bool {
        self.move_rock(Pos::down())
    }

    fn land(&mut self) {
        if let Some(rock) = &self.falling {
            assert!(!self.overlaps(rock) && !self.floats(rock));
            rock.0.iter().for_each(|pos| {
                self.rocks.insert(*pos);
            });
            self.falling = None;
            self.num_landed += 1;
        } else {
            unreachable!("No falling rock to land!");
        }
    }

    fn turn(&mut self) -> bool {
        assert!(self.falling.is_some());
        let blow = self.jets.pop_front().unwrap();
        self.blow(blow);
        self.jets.push_back(blow);
        if !self.fall() {
            self.land();
            self.next_rock();
            true
        } else { false }
    }

    fn turn_until_land(&mut self) {
        loop {
            if self.turn() {
                return;
            }
        }
    }

    fn land_n_rocks(&mut self, n: usize) {
        for _ in 0..n {
            self.turn_until_land();
        }
    }
}

fn main() {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let jets: VecDeque<Pos> = line
        .trim()
        .as_bytes()
        .iter()
        .map(|&b| match b {
            b'<' => Pos::left(),
            b'>' => Pos::right(),
            _ => unreachable!("Parse error: {b:?}"),
        })
        .collect();

    let mut chamber = Chamber::construct(7, HLine, jets.clone());
    for _ in 0..2022 {
        chamber.turn_until_land();
    }
    // println!("{}", chamber.render_top(15));
    println!("Part 1: {}", chamber.top());

    let mut chamber = Chamber::construct(7, HLine, jets.clone());
    let total_rocks = 1_000_000_000_000u64;
    // Find a number of rocks landed after which our top state repeats,
    // record how many lines are in between
    let proto_period = jets.len() * 5;
    println!("Proto_period is {}.", proto_period);
    chamber.land_n_rocks(proto_period);
    let lines_before_period = chamber.top();
    let seen = chamber.render_top(15);
    println!("Looking for:\n{}", seen);
    let mut iterations = 0;
    loop {
        chamber.land_n_rocks(proto_period);
        iterations += 1;
        println!("{}: {}/{}\n{}", iterations, chamber.num_landed, chamber.top(), chamber.render_top(15));
        if chamber.render_top(15) == seen {
            break;
        }
    }
    let period = iterations * proto_period;
    let lines_after_period = chamber.top();
    let lines_per_period = chamber.top() - lines_before_period;
    println!("After {period} rocks, we have repeated our state with {lines_per_period} extra lines");
    // Verify our period and #lines produced per period
    chamber.land_n_rocks(period);
    let lines_after_another_period = chamber.top();
    assert!(lines_after_another_period - lines_after_period == lines_per_period);
    // Now fast-forward to the end
    let remainder = total_rocks - chamber.num_landed;
    println!("We need to drop {remainder} more rocks...");
    let num_periods = remainder / (period as u64);
    let remainder = remainder % (period as u64);
    println!("Simulate {num_periods} * {period} rocks, followed by dropping a remainder of {remainder} extra rocks");
    chamber.land_n_rocks(remainder as usize);
    assert!(chamber.num_landed + num_periods * period as u64 == total_rocks);
    let total_lines = chamber.top() as u64 + lines_per_period as u64 * num_periods;
    println!("Part 2: {}", total_lines);
}
