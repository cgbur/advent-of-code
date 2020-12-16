use std::error::Error;
use std::fs::File;
use std::io::Read;
use itertools::Itertools;

fn find_combo_mul(nums: &Vec<u32>, n: usize, target: u32) -> u32 {
  nums.iter()
      .combinations(n)
      .find(|group| group.iter().map(|n| **n).sum::<u32>() == target)
      .unwrap()
      .iter().map(|n| **n)
      .product::<u32>()
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