use std::error::Error;
use std::fs::File;
use std::io::Read;

fn chars_to_binary(input: &String, zero_one: (char, char)) -> u32 {
  let (zero, one) = zero_one;
  let number = input.chars().map(|c| {
    match c {
      _ if c == zero => "0",
      _ if c == one => "1",
      _ => panic!("Bad input bit")
    }
  }).collect::<Vec<&str>>();
  u32::from_str_radix(&number.join(""), 2).unwrap()
}

fn main() -> Result<(), Box<dyn Error>> {
  let inputs = parse_input()?;

  let mut seats = inputs.iter()
      .map(|(row, col)|
          (chars_to_binary(row, ('F', 'B')),
           chars_to_binary(col, ('L', 'R'))))
      .map(|(row, col)| row * 8 + col)
      .collect::<Vec<_>>();

  let part_one = *seats.iter().max().unwrap();

  seats.sort();
  let first_idx = *seats.first().unwrap();

  let part_two = seats.iter()
      .enumerate()
      .find_map(|(index, &id)| {
        if (id - index as u32) != first_idx {
          Some(id - 1)
        } else {
          None
        }
      }).unwrap();

  println!("{:?}", part_one);
  println!("{:?}", part_two);
  Ok(())
}

fn parse_input() -> Result<Vec<(String, String)>, std::io::Error> {
  let mut input = String::new();
  File::open("./input/problem")?.read_to_string(&mut input)?;

  let inputs = input.lines().map(|line| {
    let row = (&line[..7]).to_string();
    let col = (&line[7..]).to_string();
    (row, col)
  }).collect::<Vec<_>>();

  Ok(inputs)
}
