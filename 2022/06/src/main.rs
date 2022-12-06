use std::collections::VecDeque;

use aoc::{input, Aoc};
use itertools::Itertools;

fn main() {
    // println!("part one: {}", part_one(input(), 4));
    println!("part two: {}", part_one(input(), 50));
}

fn part_one(input: &str, size: usize) -> usize {
    let mut chars = vec![None; size];
    for (idx, char) in input.chars().enumerate() {
        chars[idx % size] = Some(char);
        if chars.iter().filter_map(|c| *c).sorted().dedup().count() == chars.len() {
            return idx + 1;
        }
    }
    return 0;
}
