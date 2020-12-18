use itertools::Itertools;
use std::error::Error;
use std::fs::File;
use std::io::Read;

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

fn main() -> Result<(), Box<dyn Error>> {
  let inputs = parse_input()?;

  Ok(())
}

fn parse_input() -> Result<(), std::io::Error> {
  let mut input = String::new();
  File::open("./input/problem")?.read_to_string(&mut input)?;

  let read = input.split(&format!("{}{}", LINE_ENDING, LINE_ENDING));

  Ok(())
}
