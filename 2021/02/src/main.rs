use core::panic;
use itertools::Itertools;
use std::error::Error;
use std::fs::File;
use std::io::Read;

enum Direction {
    Forward,
    Up,
    Down,
}

type Command = (Direction, i32);

fn main() -> Result<(), Box<dyn Error>> {
    let inputs = parse_input()?;

    println!("{:?}", part_one(&inputs));
    println!("{:?}", part_two(&inputs));

    Ok(())
}

fn part_one(commands: &[Command]) -> i32 {
    let (hor, dep) = commands
        .iter()
        .fold((0, 0), |(hor, dep), (command, amt)| match command {
            Direction::Forward => (hor + amt, dep),
            Direction::Up => (hor, dep - amt),
            Direction::Down => (hor, dep + amt),
        });

    hor * dep
}

fn part_two(commands: &[Command]) -> i32 {
    let (hor, dep, _) = commands
        .iter()
        .fold((0, 0, 0), |(hor, dep, aim), (command, amt)| match command {
            Direction::Forward => (hor + amt, dep + amt * aim, aim),
            Direction::Up => (hor, dep, aim - amt),
            Direction::Down => (hor, dep, aim + amt),
        });

    hor * dep
}

fn parse_input() -> Result<Vec<Command>, std::io::Error> {
    let mut input = String::new();
    File::open("./input")?.read_to_string(&mut input)?;

    Ok(input
        .lines()
        .map(|line| {
            let mut splits = line.split_whitespace();
            let direction = match splits.next().unwrap() {
                "forward" => Direction::Forward,
                "down" => Direction::Down,
                "up" => Direction::Up,
                _ => panic!("unknown direction"),
            };
            (direction, splits.next().unwrap().parse::<i32>().unwrap()) as Command
        })
        .collect_vec())
}
