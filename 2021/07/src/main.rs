use itertools::Itertools;
use std::error::Error;
use std::fs::File;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    File::open("./input")?.read_to_string(&mut input)?;

    let crabs: Vec<i32> = input.split(',').map(|c| c.parse().unwrap()).collect_vec();

    let min_crab = *crabs.iter().min().unwrap();
    let max_crab = *crabs.iter().max().unwrap();

    let gauss_sum = |n| (n * (n + 1)) / 2;

    let part_one: i32 = (min_crab..=max_crab)
        .map(|dest| crabs.iter().map(|position| (position - dest).abs()).sum())
        .min()
        .unwrap();

    let part_two: i32 = (min_crab..=max_crab)
        .map(|dest| {
            crabs
                .iter()
                .map(|position| gauss_sum((position - dest).abs()))
                .sum()
        })
        .min()
        .unwrap();

    println!("{}", part_one);
    println!("{}", part_two);

    Ok(())
}
