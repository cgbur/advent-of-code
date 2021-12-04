use itertools::Itertools;
use std::error::Error;
use std::fs::File;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
    let inputs = parse_input()?;

    println!("{}", part_one(inputs.iter().copied()));
    println!("{}", part_two(&inputs));

    Ok(())
}

fn part_one(inputs: impl Iterator<Item = u32>) -> usize {
    inputs.tuple_windows().filter(|(a, b)| a < b).count()
}

fn part_two(inputs: &[u32]) -> usize {
    let res = inputs.iter().tuple_windows().map(|(a, b, c)| a + b + c);
    part_one(res)
}

fn parse_input() -> Result<Vec<u32>, Box<dyn Error>> {
    let mut input = String::new();
    File::open("./input")?.read_to_string(&mut input)?;

    let depths = input
        .lines()
        .map(|s| s.parse::<u32>())
        .collect::<Result<Vec<_>, _>>()?;

    Ok(depths)
}
