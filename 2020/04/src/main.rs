use std::error::Error;
use std::fs::File;
use std::io::Read;
use crate::passport::Passport;

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

mod passport;

fn main() -> Result<(), Box<dyn Error>> {
  let passes = parse_input()?;

  let part_one = passes.iter()
      .map(|p| p.is_valid_part_one() as u32)
      .sum::<u32>();

  let part_two = passes.iter()
      .map(|p| p.is_valid_part_two() as u32)
      .sum::<u32>();

  println!("{}", part_one);
  println!("{}", part_two);

  Ok(())
}

fn parse_input() -> Result<Vec<Passport>, std::io::Error> {
  let mut input = String::new();
  File::open("./input/problem")?.read_to_string(&mut input)?;

  let passes = input
      .split(&format!("{}{}", LINE_ENDING, LINE_ENDING))
      .filter(|line| line.len() > 0)
      .map(Passport::from_line)
      .collect::<Vec<_>>();

  Ok(passes)
}