use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn Error>> {
  let inputs = parse_input()?;

  println!("{}", van_eck(&inputs, 2020));
  println!("{}", van_eck(&inputs, 30_000_000));

  Ok(())
}

fn van_eck(inputs: &Vec<usize>, num_rounds: usize) -> usize {
  let mut seen = inputs.iter()
      .enumerate()
      .map(|(idx, &val)| (val, idx + 1))
      .collect::<HashMap<_, _>>();

  let last = *inputs.last().unwrap();
  (inputs.len()..num_rounds).fold(last, |last, turn| turn - seen.insert(last, turn).unwrap_or(turn))
}

fn parse_input() -> Result<Vec<usize>, std::io::Error> {
  let mut input = String::new();
  File::open("./input/problem")?.read_to_string(&mut input)?;

  let read = input
      .split(",")
      .map(|s| s.to_string())
      .map(|s| s.parse::<usize>().unwrap())
      .collect();

  Ok(read)
}
