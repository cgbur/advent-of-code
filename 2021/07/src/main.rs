use itertools::Itertools;
use std::error::Error;
use std::fs::File;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    File::open("./input")?.read_to_string(&mut input)?;

    let mut crabs: Vec<i32> = input.split(',').map(|c| c.parse().unwrap()).collect_vec();

    // Going up by 1 will increase cost for everything to the left by 1
    // and decrease to the right by 1.
    // The middle value of the sorted crabs will then minimize cost.
    // Choose the median

    // part two is the same but need the derivative of the gauss cost

    crabs.sort();

    let best = crabs[crabs.len() / 2];
    let part_one: i32 = crabs.iter().map(|pos| (pos - best).abs()).sum();

    let best = crabs.iter().sum::<i32>() / crabs.len() as i32;
    let part_two: i32 = crabs.iter().map(|pos| gauss_sum((pos - best).abs())).sum();

    println!("{}", part_one);
    println!("{}", part_two);

    brute_force(&crabs);

    Ok(())
}

fn gauss_sum(n: i32) -> i32 {
    (n * (n + 1)) / 2
}

// fast enough because small number
fn brute_force(crabs: &[i32]) {
    let min_crab = *crabs.iter().min().unwrap();
    let max_crab = *crabs.iter().max().unwrap();

    let part_one: i32 = (min_crab..=max_crab)
        .map(|dest| crabs.iter().map(|pos| (pos - dest).abs()).sum())
        .min()
        .unwrap();

    let part_two: i32 = (min_crab..=max_crab)
        .map(|dest| crabs.iter().map(|pos| gauss_sum((pos - dest).abs())).sum())
        .min()
        .unwrap();

    println!("brute force: {}", part_one);
    println!("brute force: {}", part_two);
}
