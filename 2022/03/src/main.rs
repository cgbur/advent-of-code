use aoc::input;
use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    let part_one = input()
        .lines()
        .map(|line| {
            let middle = line.len() / 2;
            let (a, b) = line.split_at(middle);
            let a = a.chars().collect::<HashSet<_>>();
            let b = b.chars().collect::<HashSet<_>>();
            let common = *a.intersection(&b).next().unwrap();
            score(common)
        })
        .sum::<u32>();

    let part_two = input()
        .lines()
        .chunks(3)
        .into_iter()
        .map(|a| {
            let (a, b, c) = a.collect_tuple().unwrap();
            let a = a.chars().collect::<HashSet<_>>();
            let b = b.chars().collect::<HashSet<_>>();
            let c = c.chars().collect::<HashSet<_>>();
            let common = a.intersection(&b).cloned().collect::<HashSet<_>>();
            let common = *common.intersection(&c).next().unwrap();
            score(common)
        })
        .sum::<u32>();

    println!("{:?}", part_one);
    println!("{:?}", part_two);
}

fn score(c: char) -> u32 {
    match c {
        'a'..='z' => c as u32 - 'a' as u32 + 1,
        'A'..='Z' => c as u32 - 'A' as u32 + 27,
        _ => panic!("Invalid character"),
    }
}
