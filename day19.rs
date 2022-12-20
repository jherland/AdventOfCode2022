use std::cmp::max;
use std::collections::HashMap;
use std::io;
use std::ops::Add;
use std::str::FromStr;

use anyhow::{Error, Result};
use text_io::scan;

use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

type Unit = u16;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct Amount(u64);

// #[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
// struct Amount {
//     ore: Unit,
//     clay: Unit,
//     obsidian: Unit,
//     geode: Unit,
// }

impl Amount {
    fn new(ore: Unit, clay: Unit, obsidian: Unit, geode: Unit) -> Self {
        Self((ore as u64) | (clay as u64) << 16 | (obsidian as u64) << 32 | (geode as u64) << 48)
    }

    fn ore(&self) -> Unit {
        (self.0 & 0xffff) as u16
    }

    fn clay(&self) -> Unit {
        (self.0 >> 16 & 0xffff) as u16
    }

    fn obsidian(&self) -> Unit {
        (self.0 >> 32 & 0xffff) as u16
    }

    fn geode(&self) -> Unit {
        (self.0 >> 48 & 0xffff) as u16
    }

    fn can_afford(&self, cost: Self) -> bool {
        self.ore() >= cost.ore()
            && self.clay() >= cost.clay()
            && self.obsidian() >= cost.obsidian()
            && self.geode() >= cost.geode()
    }

    fn pay(&self, cost: Self) -> Self {
        assert!(self.can_afford(cost));
        Self(self.0 - cost.0)
    }

    fn state(&self) -> u64 {
        self.0
    }
}

impl Add for Amount {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }

    // fn add(self, rhs: Self) -> Self {
    //     Self::new(
    //         self.ore + rhs.ore,
    //         self.clay + rhs.clay,
    //         self.obsidian + rhs.obsidian,
    //         self.geode + rhs.geode,
    //     )
    // }
}

// impl Amount {
//     fn new(ore: Unit, clay: Unit, obsidian: Unit, geode: Unit) -> Self {
//         Self { ore, clay, obsidian, geode }
//     }

//     fn can_afford(&self, cost: Self) -> bool {
//         self.ore >= cost.ore && self.clay >= cost.clay && self.obsidian >= cost.obsidian && self.geode >= cost.geode
//     }

//     fn pay(&self, cost: Self) -> Self {
//         assert!(self.can_afford(cost));
//         Self::new(
//             self.ore - cost.ore,
//             self.clay - cost.clay,
//             self.obsidian - cost.obsidian,
//             self.geode - cost.geode,
//         )
//     }

//     fn state(&self) -> u32 {
//         (self.ore as u32) | (self.clay as u32) << 8 | (self.obsidian as u32) << 16 | (self.geode as u32) << 24
//     }
// }

#[derive(Debug)]
struct Blueprint {
    id: u32,
    ore: Amount,
    clay: Amount,
    obsidian: Amount,
    geode: Amount,
}

impl FromStr for Blueprint {
    type Err = Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (id, ore_or, clay_or, obsidian_or, obsidian_cl, geode_or, geode_ob): (
            u32,
            Unit,
            Unit,
            Unit,
            Unit,
            Unit,
            Unit,
        );
        scan!(line.bytes() => "\
            Blueprint {}: \
            Each ore robot costs {} ore. \
            Each clay robot costs {} ore. \
            Each obsidian robot costs {} ore and {} clay. \
            Each geode robot costs {} ore and {} obsidian.",
            id, ore_or, clay_or, obsidian_or, obsidian_cl, geode_or, geode_ob);
        Ok(Self {
            id,
            ore: Amount::new(ore_or, 0, 0, 0),
            clay: Amount::new(clay_or, 0, 0, 0),
            obsidian: Amount::new(obsidian_or, obsidian_cl, 0, 0),
            geode: Amount::new(geode_or, 0, geode_ob, 0),
        })
    }
}

fn run_simulation(
    blueprint: &Blueprint,
    time: u8,
    inventory: Amount,
    robots: Amount,
    seen: &mut HashMap<(u8, u64, u64), Unit>,
) -> Unit {
    if time == 0 {
        return inventory.geode();
    }

    let state = (time, inventory.state(), robots.state());
    if seen.contains_key(&state) {
        return seen.get(&state).unwrap().clone();
    }

    // What are we building?
    let mut best = if inventory.can_afford(blueprint.geode) {
        let new_robots = robots + Amount::new(0, 0, 0, 1);
        run_simulation(
            blueprint,
            time - 1,
            inventory.pay(blueprint.geode) + robots,
            new_robots,
            seen,
        )
    } else if inventory.can_afford(blueprint.obsidian) {
        let new_robots = robots + Amount::new(0, 0, 1, 0);
        run_simulation(
            blueprint,
            time - 1,
            inventory.pay(blueprint.obsidian) + robots,
            new_robots,
            seen,
        )
    } else if inventory.can_afford(blueprint.clay) {
        let new_robots = robots + Amount::new(0, 1, 0, 0);
        run_simulation(
            blueprint,
            time - 1,
            inventory.pay(blueprint.clay) + robots,
            new_robots,
            seen,
        )
    } else {
        0
    };
    if inventory.can_afford(blueprint.ore) {
        let new_robots = robots + Amount::new(1, 0, 0, 0);
        best = max(
            best,
            run_simulation(
                blueprint,
                time - 1,
                inventory.pay(blueprint.ore) + robots,
                new_robots,
                seen,
            ),
        );
    }
    best = max(
        best,
        run_simulation(blueprint, time - 1, inventory + robots, robots, seen),
    );

    // // What are we building?
    // let geode = if inventory.can_afford(blueprint.geode) {
    //     let new_robots = robots + Amount::new(0, 0, 0, 1);
    //     run_simulation(blueprint, time - 1, inventory.pay(blueprint.geode) + robots, new_robots, seen)
    // } else { 0 };
    // let obsidian = if inventory.can_afford(blueprint.obsidian) {
    //     let new_robots = robots + Amount::new(0, 0, 1, 0);
    //     run_simulation(blueprint, time - 1, inventory.pay(blueprint.obsidian) + robots, new_robots, seen)
    // } else { 0 };
    // let clay = if inventory.can_afford(blueprint.clay) {
    //     let new_robots = robots + Amount::new(0, 1, 0, 0);
    //     run_simulation(blueprint, time - 1, inventory.pay(blueprint.clay) + robots, new_robots, seen)
    // } else { 0 };
    // let ore = if inventory.can_afford(blueprint.ore) {
    //     let new_robots = robots + Amount::new(1, 0, 0, 0);
    //     run_simulation(blueprint, time - 1, inventory.pay(blueprint.ore) + robots, new_robots, seen)
    // } else { 0 };
    // let nothing = run_simulation(blueprint, time - 1, inventory + robots, robots, seen);
    // let best = [nothing, geode, obsidian, clay, ore].into_iter().max().unwrap();

    seen.insert(state, best);
    best

    // for t in 1..=time {
    //     // Start building new robots
    //     let mut new_robots: Amount = Default::default();
    //     new_robots.geode += inventory.spend(&blueprint.geode);
    //     new_robots.obsidian += inventory.spend(&blueprint.obsidian);
    //     new_robots.clay += inventory.spend(&blueprint.clay);
    //     new_robots.ore += inventory.spend(&blueprint.ore);
    //     // Old robots produce one unit each
    //     inventory += robots;
    //     // New robots finish building
    //     robots += new_robots;
    //     dbg!(t, &robots, &inventory);
    // }
    // inventory.geode
}

fn main() {
    let blueprints: Vec<Blueprint> = io::stdin()
        .lines()
        .map(Result::unwrap)
        .map(|line| line.parse().unwrap())
        .collect();

    dbg!(&blueprints);

    println!(
        "Part 1: {}",
        blueprints
            .par_iter()
            .map(|bp| dbg!(run_simulation(
                bp,
                24,
                Default::default(),
                Amount::new(1, 0, 0, 0),
                &mut HashMap::new()
            )) as u32
                * dbg!(bp.id))
            .sum::<u32>()
    );

    println!(
        "Part 2: {}",
        blueprints[..3]
            .par_iter()
            .map(|bp| dbg!(run_simulation(
                bp,
                32,
                Default::default(),
                Amount::new(1, 0, 0, 0),
                &mut HashMap::new()
            )) as u32)
            .product::<u32>()
    );
    // 8410 is too low
}
