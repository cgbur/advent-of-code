use std::error::Error;
use std::fs::File;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
  let trees = parse_input()?;
  let tree_width = trees[0].len();

  let deltas = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

  let crashes = deltas.iter()
      .map(|&(dx, dy)| {
        let spots = (0..trees.len())
            .map(|index| index * dx % tree_width)
            .take(trees.len() / dy)
            .skip(1)
            .collect::<Vec<usize>>();

        let crashes = trees.iter()
            .step_by(dy)
            .skip(1)
            .zip(spots)
            .map(|(trees, index)| trees[index] as u32)
            .sum::<u32>();

        crashes
      }).collect::<Vec<_>>();

  let part_one = crashes.get(1).unwrap();
  let part_two = crashes.iter().fold(1, |acc, crashes| acc * *crashes);

  println!("{:?}", part_one);
  println!("{}", part_two);

  Ok(())
}

fn parse_input() -> Result<Vec<Vec<bool>>, std::io::Error> {
  let mut input = String::new();
  File::open("./input/problem")?.read_to_string(&mut input)?;

  let trees = input.lines()
      .map(|line| line.chars().map(|c| c == '#').collect())
      .collect::<Vec<Vec<_>>>();

  Ok(trees)
}