use itertools::Itertools;
use std::{collections::HashSet, fs::File, io::Read};

const LETTERS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() {
    let mut file = File::open("input").expect("Failed to open input.txt");
    let mut input = String::new();
    file.read_to_string(&mut input)
        .expect("Failed to read input.txt");

    let part_one = input
        .lines()
        .map(|line| {
            let middle = line.len() / 2;
            let (left, right) = line.split_at(middle);
            let left_set = left.chars().collect::<HashSet<_>>();
            let right_set = right.chars().collect::<HashSet<_>>();
            let common_char = *left_set.intersection(&right_set).next().unwrap();
            LETTERS.find(common_char).unwrap() + 1
        })
        .sum::<usize>();

    let part_two = input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|a| {
            let (a, b, c) = a.collect_tuple().unwrap();
            let a = a.chars().collect::<HashSet<_>>();
            let b = b.chars().collect::<HashSet<_>>();
            let c = c.chars().collect::<HashSet<_>>();
            let common: HashSet<_> = a.intersection(&b).cloned().collect();
            let common = *common.intersection(&c).next().unwrap();
            LETTERS.find(common).unwrap() + 1
        })
        .sum::<usize>();

    println!("{:?}", part_one);
    println!("{:?}", part_two);
}
