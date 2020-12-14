use std::error::Error;
use std::fs::File;
use std::io::Read;

struct Case {
  password: String,
  letter: String,
  low: usize,
  high: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
  let cases = parse_input();

  let part_one = cases.iter().map(|case| {
    let matches = case.password.matches(&case.letter).count();
    (case.low <= matches && matches <= case.high) as u32
  }).sum::<u32>();

  let part_two = cases.iter().map(|case| {
    let letter = case.letter.parse::<char>().unwrap();
    let a = case.password.chars().nth(case.low - 1).unwrap() == letter;
    let b = case.password.chars().nth(case.high - 1).unwrap() == letter;
    (a ^ b) as u32
  }).sum::<u32>();

  println!("{}", part_one);
  println!("{}", part_two);

  Ok(())
}

fn parse_input() -> Vec<Case> {
  let mut input = String::new();
  File::open("./input/problem").unwrap()
      .read_to_string(&mut input).unwrap();

  input.lines()
      .map(|line| {
        let mut spaces = line.split(" ");
        let mut counts = spaces
            .next().unwrap()
            .split("-");
        let low = counts
            .next().unwrap()
            .parse().unwrap();
        let high = counts
            .next().unwrap()
            .parse().unwrap();

        let letter = spaces.next().unwrap().to_string();
        let letter = letter.chars().next().unwrap().to_string();

        let password = spaces.next().unwrap().to_string();

        Case {
          password,
          letter,
          low,
          high,
        }
      }).collect::<Vec<_>>()
}