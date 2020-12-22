use itertools::Itertools;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Read;

#[cfg(windows)]
const LINE_ENDING: &str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &str = "\n";

fn part_one(chargers: &[usize]) -> usize {
  let mut jolt_level = 0;
  let mut diffs = [0; 4];

  let chargers = chargers.iter();

  for jolt in chargers {
    let diff = jolt - jolt_level;
    if diff <= 3 {
      diffs[diff] += 1;
      jolt_level = *jolt;
    } else {
      panic!("unable to make chain");
    }
  }

  diffs[1] * diffs[3]
}

fn part_two(chargers: &[usize]) -> usize {
  let mut dp = HashMap::new();
  dp.insert(0, 1);

  for &c in chargers {
    let ans = dp.get(&(c - 1)).unwrap_or(&0)
      + dp.get(&(c - 2)).unwrap_or(&0)
      + dp.get(&(c - 3)).unwrap_or(&0);
    dp.insert(c, ans);
  }

  dp[chargers.last().unwrap()]
}

fn main() -> Result<(), Box<dyn Error>> {
  let mut inputs = parse_input()?;
  inputs.sort_unstable();
  inputs.push(inputs.last().unwrap() + 3);

  let part_one = part_one(&inputs);
  let part_two = part_two(&inputs);

  println!("{:?}", part_one);
  println!("{:?}", part_two);

  Ok(())
}

fn parse_input() -> Result<Vec<usize>, std::io::Error> {
  let mut input = String::new();
  File::open("./input/problem")?.read_to_string(&mut input)?;

  let read = input
    .split(&format!("{}", LINE_ENDING))
    .map(|line| line.parse::<usize>().unwrap())
    .collect_vec();

  Ok(read)
}
