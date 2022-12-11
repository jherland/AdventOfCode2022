use std::collections::HashMap;
use std::io;
use std::str::FromStr;

use anyhow::{anyhow, Error, Result};

#[derive(Debug)]
enum Command {
    CdRoot,
    CdUp,
    CdInto(String),
    Ls,
}

impl FromStr for Command {
    type Err = Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let words: Vec<_> = line.split(' ').collect();
        match words[0] {
            "$" => match words[1] {
                "cd" => match words[2] {
                    "/" => Ok(Command::CdRoot),
                    ".." => Ok(Command::CdUp),
                    dst => Ok(Command::CdInto(dst.to_owned())),
                },
                "ls" => Ok(Command::Ls),
                _ => Err(anyhow!("Failed to parse command from {line:?}")),
            },
            _ => Err(anyhow!("Failed to parse command from {line:?}")),
        }
    }
}

#[derive(Debug)]
enum FsObject {
    Dir,
    File(usize),
}

#[derive(Debug)]
struct DirEntry {
    name: String,
    obj: FsObject,
}

impl FromStr for DirEntry {
    type Err = Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let words: Vec<_> = line.split(' ').collect();
        Ok(DirEntry {
            name: words[1].to_owned(),
            obj: match words[0] {
                "dir" => FsObject::Dir,
                _ => FsObject::File(words[0].parse()?),
            },
        })
    }
}

fn subdir(parent: &str, name: &str) -> String {
    match parent {
        "/" => "/".to_owned() + name,
        _ => parent.to_owned() + "/" + name,
    }
}

fn parent(path: &str) -> String {
    let (parent, _) = path.rsplit_once('/').unwrap();
    match parent {
        "" => "/".to_owned(),
        _ => parent.to_owned(),
    }
}

fn dir_size(dirs: &HashMap<String, Vec<DirEntry>>, path: &str) -> usize {
    assert!(dirs.contains_key(path));
    let mut sum = 0;
    for entry in dirs.get(path).unwrap() {
        match &entry.obj {
            FsObject::Dir => sum += dir_size(dirs, &subdir(path, &entry.name)),
            FsObject::File(sz) => sum += sz,
        }
    }
    sum
}

fn main() {
    let lines = io::stdin().lines().map(Result::unwrap);

    let mut dirs: HashMap<String, Vec<DirEntry>> = HashMap::new();
    let mut current = String::from("/");
    for line in lines {
        match line.parse::<Command>() {
            Ok(Command::CdRoot) => {
                // println!("   Root!");
                current = String::from("/");
                if !dirs.contains_key(&current) {
                    dirs.insert(current.clone(), Vec::new());
                }
            }
            Ok(Command::CdUp) => {
                // println!("     Up! from {:?}", current);
                current = parent(&current);
                if !dirs.contains_key(&current) {
                    dirs.insert(current.clone(), Vec::new());
                }
            }
            Ok(Command::CdInto(dst)) => {
                // println!("   Down: {:?} -> {:?}", current, dst);
                current = subdir(&current, &dst);
                if !dirs.contains_key(&current) {
                    dirs.insert(current.clone(), Vec::new());
                }
            }
            Ok(Command::Ls) => {
                // println!("     Ls!");
            }
            _ => {
                let entry = line.parse::<DirEntry>().unwrap();
                // println!("  Entry: {:?}", entry);
                dirs.get_mut(&current).unwrap().push(entry);
            }
        }
    }
    println!(
        "Part 1: {}",
        dirs.iter()
            .map(|(k, _)| dir_size(&dirs, k))
            .filter(|sz| *sz <= 100_000)
            .sum::<usize>()
    );

    let total_space = 70_000_000;
    let space_needed = 30_000_000;
    let space_used = dir_size(&dirs, "/");
    let must_free_at_least = space_used - (total_space - space_needed);
    println!(
        "Part 2: {}",
        dirs.iter()
            .map(|(k, _)| dir_size(&dirs, k))
            .filter(|sz| *sz >= must_free_at_least)
            .min()
            .unwrap()
    );
}
