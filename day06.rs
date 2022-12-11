use std::collections::HashSet;
use std::io;

fn find_first_pos_after_n_unique(s: &str, n: usize) -> Option<usize> {
    s.as_bytes()
        .windows(n)
        .enumerate()
        .map(|(i, win)| (i + n, win.iter().copied().collect::<HashSet<u8>>().len()))
        .find(|(_, unique)| *unique == n)
        .map(|(pos, _)| pos)
}

fn main() {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");

    println!(
        "Part 1: {}",
        find_first_pos_after_n_unique(&line, 4).unwrap()
    );
    println!(
        "Part 2: {}",
        find_first_pos_after_n_unique(&line, 14).unwrap()
    );
}
