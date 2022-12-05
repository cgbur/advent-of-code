use aoc::{input, Aoc};
use itertools::Itertools;

fn main() {
    let scores = input()
        .groups()
        .map(|elf| {
            elf.lines()
                .map(|num| num.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .sorted_unstable()
        .rev()
        .collect_vec();
    println!("Part 1: {}", scores.first().unwrap());
    println!("Part 2: {}", scores[0..3].iter().sum::<u32>());
}
