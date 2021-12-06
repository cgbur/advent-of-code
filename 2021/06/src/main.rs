use std::error::Error;
use std::fs::File;
use std::io::Read;

const FISHY_REPRODUCTION_MAX: usize = 9;
const FISHY_SIZE: usize = FISHY_REPRODUCTION_MAX + 1;

fn main() -> Result<(), Box<dyn Error>> {
    let inputs = parse_input()?;

    println!("{}", sim(&mut inputs.clone(), 80));
    println!("{}", sim(&mut inputs.clone(), 256));
    Ok(())
}

fn sim(fish_counts: &mut [u64], days: usize) -> u64 {
    for _ in 0..days {
        fish_counts[9] += fish_counts[0];
        fish_counts[7] += fish_counts[0];
        for i in 0..fish_counts.len() {
            fish_counts[i] = *fish_counts.get(i + 1).unwrap_or(&0);
        }
    }

    fish_counts.iter().sum()
}

fn parse_input() -> Result<[u64; FISHY_SIZE], std::io::Error> {
    let mut input = String::new();
    File::open("./input")?.read_to_string(&mut input)?;

    Ok(input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<u64>().unwrap())
        .fold([0; FISHY_SIZE], |mut acc, val| {
            acc[val as usize] += 1;
            acc
        }))
}
