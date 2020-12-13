use std::error::Error;
use std::fs::File;
use std::io::Read;
use itertools::Itertools;

fn multiply(acc: u32, val: &&u32) -> u32 {
  acc * **val
}

fn sum(acc: u32, val: &&u32) -> u32 {
  acc + **val
}

fn find_combo_mul(nums: &Vec<u32>, n: usize, target: u32) -> u32 {
  nums.iter()
      .combinations(n)
      .find(|group| group.iter().fold(0, sum) == target)
      .unwrap()
      .iter()
      .fold(1, multiply)
}

fn main() -> Result<(), Box<dyn Error>> {
  let target = 2020;

  let mut input = String::new();
  File::open("./input/problem")?.read_to_string(&mut input)?;


  let nums = input.lines()
      .map(|num| num.parse::<u32>())
      .collect::<Result<Vec<_>, _>>()?;

  println!("{}", find_combo_mul(&nums, 2, target));
  println!("{}", find_combo_mul(&nums, 3, target));

  Ok(())
}