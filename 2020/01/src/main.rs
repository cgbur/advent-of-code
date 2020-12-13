use std::fs::File;
use std::error::Error;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
  let mut input = String::new();
  File::open("./input/problem")?.read_to_string(&mut input)?;

  let nums = input.lines()
      .map(|num| num.parse::<u32>())
      .collect::<Result<Vec<_>, _>>()?;

  let part_one = nums.iter().enumerate()
      .flat_map(|(index, val)| {
        nums[index + 1..].iter().map(move |n| (val, n))
      })
      .find_map(|(a, b)| {
        if a + b == 2020 {
          Some(a * b)
        } else {
          None
        }
      }).unwrap_or(0);

  let part_two = nums.iter().enumerate()
      .flat_map(|(a_index, a)| {
        nums[a_index + 1..].iter()
            .enumerate()
            .map(move |(b_index, b)| ((a_index, a), (b_index + 1, b)))
      })
      .flat_map(|((a_index, a), (b_index, b))| {
        nums.iter()
            .enumerate()
            .filter(move |(c_index, c)| c_index != &a_index && c_index != &b_index)
            .map(move |(c_index, c)| (a, b, c))
      })
      .find_map(|(a, b, c)| {
        if a + b + c == 2020 {
          Some(a * b * c)
        } else {
          None
        }
      }).unwrap_or(0);

  println!("{:?}", part_one);
  println!("{:?}", part_two);

  Ok(())
}
