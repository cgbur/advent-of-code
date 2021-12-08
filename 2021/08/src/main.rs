// Credit for this solution goes to Axel Lindeberg (https://github.com/AxlLind)
// https://github.com/AxlLind/AdventOfCode2021/blob/main/src/bin/08.rs

use itertools::Itertools;
use std::error::Error;
use std::fs::File;
use std::io::Read;

type Digits<'a> = (Vec<&'a str>, Vec<&'a str>);

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    File::open("./input")?.read_to_string(&mut input)?;

    let input = input
        .lines()
        .map(|line| {
            let (left, right) = line.split('|').collect_tuple().unwrap();
            (
                left.split_whitespace().collect_vec(),
                right.split_whitespace().collect_vec(),
            )
        })
        .collect_vec();

    let part_one = input.iter().map(part_one).sum::<usize>();
    let part_two = input.iter().map(part_two).sum::<usize>();

    println!("{}", part_one);
    println!("{}", part_two);

    Ok(())
}

fn part_one((_, outputs): &Digits) -> usize {
    outputs
        .iter()
        .filter(|digit| matches!(digit.len(), 2 | 3 | 4 | 7))
        .count()
}

fn part_two(digits: &Digits) -> usize {
    "abcdefg"
        .chars()
        .permutations(7)
        .find_map(|perm| try_permutation(&perm, digits))
        .unwrap()
}

fn try_permutation(perm: &[char], (input, output): &Digits) -> Option<usize> {
    let invalid = input
        .iter()
        .map(|s| display_digit(perm, s))
        .any(|o| o.is_none());

    if invalid {
        return None;
    }

    let ans = output
        .iter()
        .rev()
        .enumerate()
        .map(|(i, s)| display_digit(perm, s).unwrap() * 10usize.pow(i as u32))
        .sum();

    Some(ans)
}

fn display_digit(perm: &[char], s: &str) -> Option<usize> {
    let decoded = s
        .chars()
        .map(|c| perm[(c as u8 - b'a') as usize])
        .sorted()
        .collect::<String>();

    let digit = match decoded.as_str() {
        "abcdefg" => 8,
        "bcdef" => 5,
        "acdfg" => 2,
        "abcdf" => 3,
        "abd" => 7,
        "abcdef" => 9,
        "bcdefg" => 6,
        "abef" => 4,
        "abcdeg" => 0,
        "ab" => 1,
        _ => return None,
    };

    Some(digit)
}
