use std::cmp::max;
use std::collections::HashSet;
use std::io;
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

const ROCKS: [[Pos; 5]; 5] = [
    [
        Pos { y: 0, x: 0 },
        Pos { y: 0, x: 1 },
        Pos { y: 0, x: 2 },
        Pos { y: 0, x: 3 },
        Pos { y: 0, x: 0 },
    ],
    [
        Pos { y: 0, x: 1 },
        Pos { y: 1, x: 0 },
        Pos { y: 1, x: 1 },
        Pos { y: 1, x: 2 },
        Pos { y: 2, x: 1 },
    ],
    [
        Pos { y: 0, x: 0 },
        Pos { y: 0, x: 1 },
        Pos { y: 0, x: 2 },
        Pos { y: 1, x: 2 },
        Pos { y: 2, x: 2 },
    ],
    [
        Pos { y: 0, x: 0 },
        Pos { y: 1, x: 0 },
        Pos { y: 2, x: 0 },
        Pos { y: 3, x: 0 },
        Pos { y: 0, x: 0 },
    ],
    [
        Pos { y: 0, x: 0 },
        Pos { y: 0, x: 1 },
        Pos { y: 1, x: 0 },
        Pos { y: 1, x: 1 },
        Pos { y: 0, x: 0 },
    ],
];

#[derive(Clone, Debug)]
struct Rock([Pos; 5]);

impl Rock {
    fn from(raw: [Pos; 5]) -> Self {
        Self(raw.clone())
    }

    fn translate(&self, delta: Pos) -> Self {
        Self([
            self.0[0] + delta,
            self.0[1] + delta,
            self.0[2] + delta,
            self.0[3] + delta,
            self.0[4] + delta,
        ])
    }
}

struct Chamber {
    rocks: HashSet<Pos>,
    falling: Option<Rock>,
    jets: Vec<Pos>,
    num_turns: u64,
    num_landed: u64,
}

impl Chamber {
    fn construct(jets: Vec<Pos>) -> Self {
        let mut ret = Self {
            rocks: HashSet::new(),
            falling: None,
            jets,
            num_turns: 0,
            num_landed: 0,
        };
        ret.next_rock();
        ret
    }

    fn top(&self) -> i32 {
        self.rocks
            .iter()
            .map(|pos| pos.y + 1)
            .max()
            .unwrap_or_default()
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
            Some(rock) => rock.0.iter().copied().collect(),
            None => HashSet::new(),
        };
        for y in ((self.top_w_falling() - (lines as i32))..=self.top_w_falling()).rev() {
            ret.push_str("|");
            for x in 0..7 {
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
        for _ in 0..7 {
            ret.push_str("-");
        }
        ret.push_str("+\n");
        ret
    }

    fn render(&self) -> String {
        self.render_top(self.top_w_falling() as usize)
    }

    fn overlaps(&self, rock: &Rock) -> bool {
        rock.0
            .iter()
            .any(|pos| pos.x < 0 || pos.x >= 7 || pos.y < 0 || self.rocks.contains(pos))
    }

    fn floats(&self, rock: &Rock) -> bool {
        !self.overlaps(&rock.translate(Pos::down()))
    }

    fn next_rock(&mut self) {
        assert!(self.falling.is_none());
        let start_pos = Pos::new(self.top() + 3, 2);
        let rock = Rock::from(ROCKS[self.num_landed as usize % ROCKS.len()]);
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
        let blow = self.jets[self.num_turns as usize % self.jets.len()];
        self.blow(blow);
        self.num_turns += 1;
        if !self.fall() {
            self.land();
            self.next_rock();
            true
        } else {
            false
        }
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
    let jets: Vec<Pos> = line
        .trim()
        .as_bytes()
        .iter()
        .map(|&b| match b {
            b'<' => Pos::left(),
            b'>' => Pos::right(),
            _ => unreachable!("Parse error: {b:?}"),
        })
        .collect();

    let mut chamber = Chamber::construct(jets.clone());
    for _ in 0..2022 {
        chamber.turn_until_land();
    }
    // println!("{}", chamber.render_top(15));
    println!("Part 1: {}", chamber.top());

    let mut chamber = Chamber::construct(jets.clone());
    let total_rocks = 1_000_000_000_000u64;
    // Find a number of rocks landed after which our top state repeats,
    // record how many lines are in between
    let proto_period = jets.len() * 5;
    // println!("Proto_period is {}.", proto_period);
    chamber.land_n_rocks(proto_period);
    let lines_before_period = chamber.top();
    let seen = chamber.render_top(15);
    // println!("Looking for:\n{}", seen);
    let mut iterations = 0;
    loop {
        chamber.turn_until_land();
        iterations += 1;
        if chamber.render_top(15) == seen {
            // println!("{}: {}/{}\n{}", iterations, chamber.num_landed, chamber.top(), chamber.render_top(15));
            break;
        }
    }
    let period = iterations;
    let lines_after_period = chamber.top();
    let lines_per_period = chamber.top() - lines_before_period;
    // println!("After {period} rocks, we have repeated our state with {lines_per_period} extra lines");
    // Verify our period and #lines produced per period
    chamber.land_n_rocks(period);
    let lines_after_another_period = chamber.top();
    assert!(lines_after_another_period - lines_after_period == lines_per_period);
    // Now fast-forward to the end
    let remainder = total_rocks - chamber.num_landed;
    // println!("We need to drop {remainder} more rocks...");
    let num_periods = remainder / (period as u64);
    let remainder = remainder % (period as u64);
    // println!("Simulate {num_periods} * {period} rocks, followed by dropping a remainder of {remainder} extra rocks");
    chamber.land_n_rocks(remainder as usize);
    assert!(chamber.num_landed + num_periods * period as u64 == total_rocks);
    let total_lines = chamber.top() as u64 + lines_per_period as u64 * num_periods;
    println!("Part 2: {}", total_lines);
}
