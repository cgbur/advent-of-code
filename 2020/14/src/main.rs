use std::error::Error;
use std::fs::File;
use std::io::{Read};
use std::collections::HashMap;
use crate::MaskOrMem::{Mask, Mem};

#[derive(Clone)]
pub enum MaskOrMem {
  Mask(Vec<Option<bool>>),
  Mem((u64, u64)),
}

fn main() -> Result<(), Box<dyn Error>> {
  let inputs = parse_input()?;

  println!("{}", part_one(inputs.clone()));
  println!("{}", part_two(inputs.clone()));

  Ok(())
}

pub fn part_one_mask(mask: &Vec<Option<bool>>, val: u64) -> u64 {
  let value = mask.iter()
      .enumerate()
      .map(|(i, v)| match v {
        None => (val >> mask.len() - 1 - i) & 0b1,
        Some(n) => *n as u64
      })
      .map(|n| n.to_string())
      .collect::<Vec<String>>().join("");

  u64::from_str_radix(&value, 2).unwrap()
}

pub fn part_two_mask(mask: &Vec<Option<bool>>, val: u64) -> Vec<u64> {
  let addrs = mask.iter()
      .enumerate()
      .map(|(i, v)| match v {
        None => None,
        Some(n) => match *n {
          true => Some(1),
          false => Some((val >> mask.len() - 1 - i) & 0b1)
        }
      }).collect::<Vec<Option<_>>>();

  let floating_indices = addrs.iter()
      .enumerate()
      .filter_map(|(index, bit)|
          if bit.is_none() {
            Some(index)
          } else {
            None
          })
      .collect::<Vec<_>>();

  let enumerate_addresses = (0..1 << floating_indices.len())
      .map(|bits| {
        let mut current_bit = floating_indices.len();
        let mut num = addrs.clone();

        for index in &floating_indices {
          num[*index] = Some((bits >> current_bit - 1) & 1);
          current_bit -= 1;
        }

        num
      }).collect::<Vec<_>>();

  let addresses = enumerate_addresses.iter()
      .map(|item| {
        let bitstring = item.iter()
            .map(|k| k.unwrap().to_string())
            .collect::<Vec<String>>()
            .join("");
        u64::from_str_radix(&bitstring, 2).unwrap()
      }).collect::<Vec<_>>();

  addresses
}

fn part_one(inputs: Vec<MaskOrMem>) -> u64 {
  let mut memory = HashMap::new();
  let mut current_mask = Vec::new();

  for input in inputs {
    match input {
      Mask(mask) => current_mask = mask,
      Mem((addr, val)) => {
        memory.insert(addr, part_one_mask(&current_mask, val));
      }
    }
  }

  memory.values().sum::<u64>()
}


fn part_two(inputs: Vec<MaskOrMem>) -> u64 {
  let mut memory = HashMap::new();
  let mut current_mask = Vec::new();

  for input in inputs {
    match input {
      Mask(mask) => current_mask = mask,
      Mem((addr, val)) => {
        for addr in part_two_mask(&current_mask, addr) {
          memory.insert(addr, val);
        };
      }
    }
  }

  memory.values().sum::<u64>()
}

fn parse_input() -> Result<Vec<MaskOrMem>, std::io::Error> {
  let mut input = String::new();
  File::open("./input/problem")?.read_to_string(&mut input)?;

  let mut inputs = Vec::new();
  for line in input.lines() {
    if line.contains("mask") {
      let mask = line.split("=")
          .nth(1)
          .unwrap()
          .trim()
          .chars()
          .map(|c| match c {
            'X' => None,
            _ => Some(c.to_string().parse::<u32>().unwrap() == 1)
          })
          .collect::<Vec<Option<bool>>>();
      inputs.push(Mask(mask))
    } else {
      let addr = line[4..].split("]").next().unwrap().parse::<u64>().unwrap();
      let val = line.split(" = ").nth(1).unwrap().parse::<u64>().unwrap();

      inputs.push(Mem((addr, val)));
    }
  }


  Ok(inputs)
}
