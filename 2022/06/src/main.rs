use std::collections::VecDeque;

use aoc::{input, Aoc};
use itertools::Itertools;

fn main() {
    println!("part one: {}", part_one(input(), 4));
    println!("part two: {}", part_one(input(), 14));
}

fn part_one(input: &str, size: usize) -> usize {
    let mut chars = VecDeque::from(vec![None; size]);
    for (idx, char) in input.chars().enumerate() {
        chars.pop_front();
        chars.push_back(Some(char));
        if chars.iter().filter_map(|c| *c).sorted().dedup().count() == chars.len() {
            return idx + 1;
        }
    }
    panic!("No solution found");
}
