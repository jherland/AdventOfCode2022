use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::io;
use std::str::FromStr;

use anyhow::{Error, Result};
use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
struct Valve {
    name: String,
    flow: u32,
    conns: HashSet<String>,
}

impl FromStr for Valve {
    type Err = Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let input_re = Regex::new(
            r"^Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? ([A-Z ,]+)$",
        )
        .unwrap();
        let captures = input_re.captures(line).unwrap();
        let conns = captures[3]
            .split(',')
            .map(|s| s.trim().to_owned())
            .collect();
        Ok(Self {
            name: captures[1].to_owned(),
            flow: captures[2].parse().unwrap(),
            conns,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct OpenValves(u64);

impl OpenValves {
    fn from_indices(indices: Vec<u32>) -> Self {
        assert!(indices.iter().all(|i| *i < 64));
        Self(indices.iter().map(|i| 1 << i).sum())
    }

    fn has(self, valve: u32) -> bool {
        assert!(valve < 64);
        self.0 & (1 << valve) > 0
    }

    fn add(self, valve: u32) -> Self {
        assert!(valve < 64);
        Self(self.0 | (1 << valve))
    }
}

struct World {
    valve_index_by_name: HashMap<String, u32>,
    flow_map: Vec<u32>,
    conn_map: Vec<Vec<u32>>,
    state_v_bits: usize,
    state_opened_bits: usize,
    state_t_bits: usize,
    state_player_bits: usize,
}

impl World {
    fn bits_needed(max_value: usize) -> usize {
        (0..).find(|n| max_value < 1 << n).unwrap()
    }

    fn construct(parsed_valves: Vec<Valve>, max_t: u32, max_players: usize) -> Self {
        let state_v_bits = Self::bits_needed(parsed_valves.len() - 1);
        let state_t_bits = Self::bits_needed(max_t as usize);
        let state_player_bits = Self::bits_needed(max_players);

        // Map valve names to correspoding index in 'valves'
        let valve_index_by_name: HashMap<String, u32> = parsed_valves
            .iter()
            .enumerate()
            .map(|(i, v)| (v.name.to_owned(), i as u32))
            .collect();

        // Build flow map and connections map based on valve indices
        let flow_map: Vec<u32> = parsed_valves.iter().map(|v| v.flow).collect();
        let conn_map = parsed_valves
            .iter()
            .map(|v| {
                v.conns
                    .iter()
                    .map(|name| *valve_index_by_name.get(name).unwrap())
                    .sorted()
                    .collect()
            })
            .collect();

        // Goal is to have opened all valves with flow rate > 0
        let end_goal = OpenValves::from_indices(
            flow_map
                .iter()
                .enumerate()
                .filter_map(|(i, flow)| if *flow > 0 { Some(i as u32) } else { None })
                .collect(),
        );

        let state_opened_bits = Self::bits_needed(end_goal.0 as usize);

        // Make sure the full state will fit in a u64
        assert!(state_v_bits + state_opened_bits + state_t_bits + state_player_bits <= 64);

        Self {
            valve_index_by_name,
            flow_map,
            conn_map,
            state_v_bits,
            state_opened_bits,
            state_t_bits,
            state_player_bits,
        }
    }

    fn get(&self, name: &str) -> u32 {
        *self.valve_index_by_name.get(name).unwrap() as u32
    }

    fn state(&self, v: u32, opened: OpenValves, t: u32, player: usize) -> usize {
        // How many possible states are there?
        // - 'v' can be any one of the valves (self.parsed_valves.len())
        // - 'opened' can be any subset of open valves (2^(self.flow_map.len()))
        // - 't' is anywhere between the start time and 0 (30)
        // - 'player' have 2 possible states
        // Answer is the product of these
        assert!(Self::bits_needed(v as usize) <= self.state_v_bits);
        assert!(Self::bits_needed(opened.0 as usize) <= self.state_opened_bits);
        assert!(Self::bits_needed(t as usize) <= self.state_t_bits);
        assert!(Self::bits_needed(player) <= self.state_player_bits);
        (opened.0 as usize)
            | (t as usize) << self.state_opened_bits
            | (v as usize) << (self.state_t_bits + self.state_opened_bits)
            | player << (self.state_v_bits + self.state_t_bits + self.state_opened_bits)
    }

    fn run(&self, start: u32, opened: OpenValves, t0: u32, player: usize) -> u32 {
        let bits_needed =
            self.state_v_bits + self.state_opened_bits + self.state_t_bits + self.state_player_bits;
        let mut states: Vec<u32> = vec![0; 1 << bits_needed];

        // If I am at valve 'v', and I've opened the set of valves 'opened', and I
        // have 't' minutes left, and there are 'player' other players acting after
        // me, then how many points can I score from this position?
        fn score(
            v: u32,
            opened: OpenValves,
            t: u32,
            player: usize,
            states: &mut Vec<u32>,
            conditions: &(&World, u32, u32),
        ) -> u32 {
            let (world, start, t0) = conditions;

            if t == 0 {
                if player > 0 {
                    return score(*start, opened, *t0, player - 1, states, conditions);
                } else {
                    return 0;
                }
            }

            // Check if we've been in a similar situation before
            let state = world.state(v, opened, t, player);
            if states[state] != 0 {
                return states[state];
            }

            // Evaluate possible actions:
            let mut ret = 0;
            // Open current valve if it is not open, and worth opening
            if !opened.has(v) && world.flow_map[v as usize] > 0 {
                let new_opened = opened.add(v);
                assert!(new_opened > opened);
                ret = (t - 1) * world.flow_map[v as usize]
                    + score(v, new_opened, t - 1, player, states, conditions);
            }
            // Move to a neighboring valve
            for neighbor in &world.conn_map[v as usize] {
                ret = max(
                    ret,
                    score(*neighbor, opened, t - 1, player, states, conditions),
                );
            }

            // Record this situation in case we end up here again
            states[state] = ret;
            ret
        }

        score(start, opened, t0, player, &mut states, &(self, start, t0))
    }
}

fn main() {
    let valves: Vec<Valve> = io::stdin()
        .lines()
        .map(Result::unwrap)
        .map(|line| line.parse().unwrap())
        .sorted_by_key(|v: &Valve| v.flow)
        .rev()
        .collect();
    let world = World::construct(valves, 30, 1);
    let start = world.get("AA");

    println!("Part 1: {}", world.run(start, OpenValves(0), 30, 0));
    println!("Part 2: {}", world.run(start, OpenValves(0), 26, 1));
}
