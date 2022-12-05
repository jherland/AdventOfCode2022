use std::io;
use std::iter::zip;
use std::str::FromStr;

use anyhow::Error;

type Crate = u8;

type Stack = Vec<Crate>;

#[derive(Clone, Debug)]
struct Stacks(Vec<Stack>);

impl Stacks {
    fn parse<'a, I>(lines: I) -> Stacks
    where
        I: Iterator<Item = &'a String>,
    {
        let mut ret = Stacks(Vec::new());
        for line in lines {
            if line.starts_with(" 1") {
                break; // column legend
            }
            let columns: Vec<_> = line.as_bytes().chunks(4).collect();
            while ret.0.len() < columns.len() {
                ret.0.push(Stack::new());
            }
            for (col, stack) in zip(columns, ret.0.iter_mut()) {
                let col = std::str::from_utf8(col).unwrap().trim();
                if !col.is_empty() {
                    assert!(col.len() == 3 && col.starts_with('[') && col.ends_with(']'));
                    stack.insert(0, col.as_bytes()[1]);
                }
            }
        }
        ret
    }

    fn lifo_move(&mut self, m: &Move) {
        assert!(m.src < self.0.len() && m.dst < self.0.len());
        assert!(m.amt <= self.0[m.src].len());
        for _ in 0..m.amt {
            let item = self.0[m.src].pop().unwrap();
            self.0[m.dst].push(item);
        }
    }

    fn fifo_move(&mut self, m: &Move) {
        assert!(m.src < self.0.len() && m.dst < self.0.len());
        let stack = &mut self.0[m.src];
        assert!(m.amt <= stack.len());
        let new_stack_len = stack.len() - m.amt;
        let mut crates: Vec<Crate> = stack[new_stack_len..].to_owned();
        stack.truncate(new_stack_len);
        self.0[m.dst].append(&mut crates);
    }

    fn tops(&self) -> Vec<Crate> {
        self.0.iter().map(|stack| *stack.last().unwrap()).collect()
    }
}

#[derive(Debug)]
struct Move {
    amt: usize,
    src: usize,
    dst: usize,
}

impl FromStr for Move {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words: Vec<_> = s.split_ascii_whitespace().collect();
        assert!(words.len() == 6 && words[0] == "move" && words[2] == "from" && words[4] == "to");
        let amt = words[1].parse()?;
        let mut src = words[3].parse()?;
        let mut dst = words[5].parse()?;
        assert!(src > 0 && dst > 0); // Turn src/dst from ordinals into indices
        src -= 1;
        dst -= 1;
        Ok(Move { amt, src, dst })
    }
}

fn main() {
    let lines: Vec<_> = io::stdin().lines().map(Result::unwrap).collect();
    let parts: Vec<_> = lines.split(String::is_empty).collect();
    assert!(parts.len() == 2);

    let stacks = Stacks::parse(parts[0].iter());
    let moves: Vec<Move> = parts[1].iter().map(|s| s.parse().unwrap()).collect();

    let mut stacks1 = stacks.clone();
    moves.iter().for_each(|m| stacks1.lifo_move(m));
    println!(
        "Part 1: {}",
        std::str::from_utf8(stacks1.tops().as_slice()).unwrap()
    );

    let mut stacks2 = stacks;
    moves.iter().for_each(|m| stacks2.fifo_move(m));
    println!(
        "Part 2: {}",
        std::str::from_utf8(stacks2.tops().as_slice()).unwrap()
    );
}
