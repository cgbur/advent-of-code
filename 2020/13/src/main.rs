use std::error::Error;
use std::fs::File;
use std::io::{Read};
use std::env;

struct Bus {
  index: u64,
  id: u64,
}

fn main() -> Result<(), Box<dyn Error>> {
  let (time, buses) = parse_input()?;

  println!("{}", part_one(time, &buses));
  println!("{}", part_two(&buses));
  Ok(())
}

fn part_one(time: u64, buses: &Vec<Bus>) -> u64 {
  let (best_index, best_wait) = buses.iter()
      .map(|bus| bus.id)
      .map(|bus| (((time as f32 / bus as f32).ceil() as u64) * bus) - time)
      .enumerate()
      .min_by_key(|&(_, time)| time)
      .unwrap();

  best_wait * buses[best_index].id
}

/// [Chinese remainder theorem](https://en.wikipedia.org/wiki/Chinese_remainder_theorem)
fn part_two(buses: &Vec<Bus>) -> u64 {
  let mut result = 1;
  let mut mode = 1;

  for bus in buses {
    while (result + bus.index) % bus.id != 0 {
      result += mode;
    }
    mode *= bus.id
  }

  result
}

fn parse_input() -> Result<(u64, Vec<Bus>), std::io::Error> {
  let args: Vec<String> = env::args().collect();
  let input_file = match args.len() {
    2 => &args[1],
    _ => "./input/problem"
  };

  let mut input = String::new();
  File::open(input_file)?.read_to_string(&mut input)?;

  let mut input = input.lines();
  let time = input.next().unwrap().parse::<u64>().unwrap();
  let buses = input.next().unwrap().split(",")
      .enumerate()
      .filter(|&(_index, bus)| bus != "x")
      .map(|(index, bus)| Bus { index: index as u64, id: bus.parse::<u64>().unwrap() })
      .collect::<Vec<_>>();

  Ok((time, buses))
}