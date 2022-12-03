use aoc::input;
use itertools::Itertools;

fn main() {
    let part_one = input()
        .lines()
        .map(|line| {
            let (a, b) = line.split_at(line.len() / 2);
            score(&intersect(a, b))
        })
        .sum::<u32>();

    let part_two = input()
        .lines()
        .tuples()
        .map(|(a, b, c)| score(&intersect(&intersect(a, b), c)))
        .sum::<u32>();

    println!("{:?}", part_one);
    println!("{:?}", part_two);
}

fn intersect(a: &str, b: &str) -> String {
    a.chars().filter(|&c| b.contains(c)).collect()
}

fn score(c: &str) -> u32 {
    let c = c.chars().next().unwrap();
    match c {
        'a'..='z' => c as u32 - 'a' as u32 + 1,
        'A'..='Z' => c as u32 - 'A' as u32 + 27,
        _ => panic!("Invalid character"),
    }
}
