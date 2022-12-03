use std::io;

fn main() {
    let mut elves = Vec::<u32>::new();

    let mut calories = 0;
    for line in io::stdin().lines() {
        let line = line.unwrap();
        if line.len() == 0 {
            elves.push(calories);
            calories = 0;
        } else {
            calories += line.parse::<u32>().expect("Failed to parse {line:?}");
        }
    }
    elves.sort();

    // part 1
    let max_elf = elves[elves.len() - 1];
    println!("{max_elf:?}");

    // part 2
    let max_elves: u32 = elves[elves.len() - 3..].iter().sum();
    println!("{max_elves:?}");
}
