use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn Error>> {
  let inputs = parse_input()?;

  println!("{:?}", van_eck(&inputs, 2020));
  println!("{:?}", van_eck(&inputs, 30_000_000));

  Ok(())
}

fn van_eck(inputs: &Vec<u32>, num_rounds: u32) -> u32 {
  let turn_start = (inputs.len() - 1) as u32;
  let turn_end = num_rounds - 1;

  let mut memory = HashMap::new();

  for (idx, num) in inputs.iter().enumerate() {
    memory.insert(*num, idx as u32);
  }

  let mut last_spoken = *inputs.last().unwrap();

  for turn in turn_start..turn_end {
    let last = memory.entry(last_spoken).or_insert(turn);
    if *last == turn {
      last_spoken = 0;
    } else {
      last_spoken = turn - *last;
      *last = turn;
    }
  }

  *memory.entry(last_spoken).key()
}

fn parse_input() -> Result<Vec<u32>, std::io::Error> {
  let mut input = String::new();
  File::open("./input/problem")?.read_to_string(&mut input)?;

  let read = input
      .split(",")
      .map(|s| s.to_string())
      .map(|s| s.parse::<u32>().unwrap())
      .collect();

  Ok(read)
}
