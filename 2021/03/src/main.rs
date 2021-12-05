use core::panic;
use itertools::Itertools;
use std::error::Error;
use std::fs::File;
use std::io::Read;

/// Not my finest hour

fn main() -> Result<(), Box<dyn Error>> {
    let inputs = parse_input()?;

    println!("{}", part_one(&inputs));
    println!("{}", part_two(&inputs));

    Ok(())
}

fn part_one(inputs: &[Vec<u8>]) -> usize {
    let gamma = (0..inputs[0].len())
        .map(|idx| inputs.iter().map(move |inner| inner[idx]).collect_vec())
        .map(|bits| zero_or_one(&bits).to_string())
        .join("");

    let size = gamma.len() as u32;
    let gamma = usize::from_str_radix(&gamma, 2).unwrap();
    let epsilon = !(gamma << (usize::BITS - size)) >> (usize::BITS - size);

    gamma * epsilon
}

fn part_two(inputs: &[Vec<u8>]) -> usize {
    let oxygen = part_two_helper(inputs, true);
    let co2 = part_two_helper(inputs, false);

    let oxygen = usize::from_str_radix(&oxygen, 2).unwrap();
    let co2 = usize::from_str_radix(&co2, 2).unwrap();

    oxygen * co2
}

fn part_two_helper(inputs: &[Vec<u8>], is_max: bool) -> String {
    let mut inputs = inputs.iter().cloned().collect_vec();
    let mut index = 0;

    while inputs.len() > 1 {
        let mut common_bit = zero_or_one(&inputs.iter().map(|vec| vec[index]).collect_vec());

        if !is_max {
            common_bit = !(common_bit == 1) as u8
        }

        let remove_positions = inputs
            .iter()
            .positions(|bits| bits[index] == common_bit)
            .rev()
            .collect_vec();

        inputs = inputs
            .iter()
            .enumerate()
            .filter(|(idx, _val)| remove_positions.contains(idx))
            .map(|(_a, b)| b.clone())
            .collect_vec();

        index += 1;
    }

    inputs[0].iter().join("")
}

fn zero_or_one(numbers: &[u8]) -> u8 {
    let mut zero = 0;
    let mut one = 0;
    for num in numbers {
        match num {
            0 => zero += 1,
            1 => one += 1,
            _ => panic!("sef"),
        }
    }

    (one >= zero) as u8
}

fn parse_input() -> Result<Vec<Vec<u8>>, std::io::Error> {
    let mut input = String::new();
    File::open("./input")?.read_to_string(&mut input)?;

    Ok(input
        .lines()
        .map(|line| line.chars().map(|c| c as u8 - b'0').collect_vec())
        .collect_vec())
}
