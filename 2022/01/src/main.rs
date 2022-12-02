use itertools::Itertools;
use std::{fs::File, io::Read};

fn main() {
    let scores = input();
    println!("Part 1: {}", scores.first().unwrap());
    println!("Part 2: {}", scores[0..3].iter().sum::<u32>());
}

fn input() -> Vec<u32> {
    let mut file = File::open("test").expect("Failed to open input.txt");
    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .expect("Failed to read input.txt");

    let elves = buf
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|num| num.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .sorted_unstable()
        .rev()
        .collect_vec();
    elves
}
