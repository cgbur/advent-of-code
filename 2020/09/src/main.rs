use itertools::Itertools;
use std::error::Error;
use std::fs::File;
use std::io::Read;

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

const LOOKBACK: usize = 25;

fn is_a_sum(num: &usize, array: &[usize]) -> bool {
  let val = array
    .iter()
    .combinations(2)
    .find(|combo| combo[0] + combo[1] == *num)
    .is_some();
  val
}

fn part_two(target: usize, nums: &[usize]) -> usize {
  let (mut start, mut end, mut sum) = (0, 0, 0);
  while sum != target {
    if sum < target {
      sum += nums[end];
      end += 1;
    } else {
      sum -= nums[start];
      start += 1;
    }
  }
  let &(min, max) = &nums[start..end].iter().minmax().into_option().unwrap();
  min + max
}

fn main() -> Result<(), Box<dyn Error>> {
  let inputs = parse_input()?;

  let (_, part_one) = inputs
    .iter()
    .enumerate()
    .skip(LOOKBACK)
    .find(|&(idx, val)| !is_a_sum(val, &inputs[idx - LOOKBACK..idx]))
    .unwrap_or_else(|| panic!("bad pattern not found"));

  let part_two = part_two(*part_one, &inputs);

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
