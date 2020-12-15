use std::error::Error;
use std::fs::File;
use std::io::Read;
use itertools::Itertools;
use std::collections::HashMap;

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

fn main() -> Result<(), Box<dyn Error>> {
  let inputs = parse_input()?;

  let part_one = inputs.iter()
      .map(|group| group.iter().flatten().collect_vec())
      .map(|group| group.iter().unique().count() as u32)
      .sum::<u32>();

  let part_two = inputs.iter()
      .map(|group| group.iter()
          .fold(HashMap::with_capacity(26), |mut acc, votes| {
            for vote in votes {
              let counter = acc.entry(vote).or_insert(0);
              *counter += 1;
            }
            acc
          })
          .values()
          .filter(|&&count| count == group.len())
          .count() as u32
      ).sum::<u32>();

  println!("{:?}", part_one);
  println!("{:?}", part_two);
  
  Ok(())
}

fn parse_input() -> Result<Vec<Vec<Vec<char>>>, std::io::Error> {
  let mut input = String::new();
  File::open("./input/problem")?.read_to_string(&mut input)?;

  let responses = input
      .split(&format!("{}{}", LINE_ENDING, LINE_ENDING))
      .map(|group| group
          .split(LINE_ENDING)
          .map(|line| line.chars().collect::<Vec<char>>())
          .collect_vec())
      .collect_vec();

  Ok(responses)
}
