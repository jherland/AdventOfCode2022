use std::io;

fn find_common(a: &[u8], b: &[u8]) -> Option<u8> {
    for c in a {
        for d in b {
            if c == d {
                return Some(*c);
            }
        }
    }
    None
}

fn find_commons(a: &[u8], b: &[u8]) -> Vec<u8> {
    let mut ret: Vec<u8> = Vec::new();
    for c in a {
        for d in b {
            if c == d {
                ret.push(*c);
            }
        }
    }
    ret
}

fn score(c: u8) -> u32 {
    if c >= b'a' && c <= b'z' {
        1 + (c - b'a') as u32
    } else if c >= b'A' && c <= b'Z' {
        27 + (c - b'A') as u32
    } else {
        unreachable!("Unrecognized letter {c:?}!")
    }
}

fn main() {
    let mut lines: Vec<String> = Vec::new();
    for line in io::stdin().lines() {
        lines.push(line.expect("No line?"));
    }

    // part 1
    let mut sum = 0;
    for line in &lines {
        let len = line.trim().len();
        assert!(len % 2 == 0);
        let first = line[..len / 2].as_bytes();
        let second = line[len / 2..].as_bytes();
        sum += match find_common(first, second) {
            Some(c) => score(c),
            None => unreachable!("Found no common item in {line:?}!"),
        };
    }
    println!("{sum}");

    // part 2
    sum = 0;
    for chunk in lines.chunks(3) {
        match chunk {
            [a, b, c] => {
                let commons = find_commons(b.as_bytes(), c.as_bytes());
                let common = find_common(a.as_bytes(), &commons);
                sum += score(common.unwrap());
            }
            _ => unreachable!("Must be multiple of 3 lines!"),
        }
    }
    println!("{sum}");
}
