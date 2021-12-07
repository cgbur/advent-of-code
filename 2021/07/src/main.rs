use itertools::Itertools;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    File::open("./input")?.read_to_string(&mut input)?;

    let crabs = input.split(',').map(|c| c.parse().unwrap()).collect_vec();

    let crab_map = crabs.iter().fold(HashMap::new(), |mut acc, val| {
        *acc.entry(*val).or_insert(0) += 1;
        acc
    });

    println!("{}", best_position(&crabs, &crab_map, cost));
    println!("{}", best_position(&crabs, &crab_map, cost_two));

    Ok(())
}

fn best_position(
    crabs: &[u32],
    crab_map: &HashMap<u32, u32>,
    cost_fn: impl Fn(usize, &HashMap<u32, u32>) -> u32,
) -> u32 {
    let mut costs = vec![0; *crabs.iter().max().unwrap() as usize];

    for (i, cost) in costs.iter_mut().enumerate() {
        *cost = cost_fn(i, crab_map);
    }

    *costs.iter().min().unwrap()
}

fn cost(position: usize, crab_map: &HashMap<u32, u32>) -> u32 {
    let mut cost = 0;

    for (pos, amt) in crab_map.iter() {
        cost += (position as i32 - *pos as i32).abs() as u32 * amt;
    }

    cost
}

fn cost_two(position: usize, crab_map: &HashMap<u32, u32>) -> u32 {
    let mut cost = 0;

    for (pos, amt) in crab_map.iter() {
        // geometric series summing: Let the compiler do the work
        cost += (0..=(position as i32 - *pos as i32).abs()).sum::<i32>() as u32 * amt;
    }

    cost
}
