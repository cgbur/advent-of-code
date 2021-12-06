use std::error::Error;
use std::fs::File;
use std::io::Read;

const FISHY_REPRODUCTION_MAX: usize = 8;
const FISHY_REPRODUCTION: usize = 6;
const FISHY_SIZE: usize = FISHY_REPRODUCTION_MAX + 1;

fn main() -> Result<(), Box<dyn Error>> {
    let mut inputs = parse_input()?;

    println!("{}", sim(&mut inputs.clone(), 80));
    println!("{}", sim(&mut inputs, 256));
    Ok(())
}

fn sim(fish_counts: &mut [u64], days: usize) -> u64 {
    for _ in 0..days {
        fish_counts.rotate_left(1);
        fish_counts[FISHY_REPRODUCTION] += fish_counts[FISHY_REPRODUCTION_MAX];
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
