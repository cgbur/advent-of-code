use crate::Instruction::{Acc, Jmp, Nop};
use itertools::Itertools;
use std::error::Error;
use std::fs::File;
use std::io::Read;

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

#[derive(Debug, PartialEq, Copy, Clone)]
enum Instruction {
  Acc(i32),
  Jmp(i32),
  Nop(i32),
}

struct Program {
  pc: usize,
  acc: i32,
  instructions: Vec<Instruction>,
  has_executed: Vec<bool>,
}

impl Program {
  pub fn new(instructions: Vec<Instruction>) -> Self {
    Self {
      pc: 0,
      acc: 0,
      has_executed: vec![false; instructions.len()],
      instructions,
    }
  }

  pub fn execute_instruction(&mut self) {
    match self.instructions[self.pc] {
      Acc(val) => self.acc += val,
      Jmp(offset) => self.pc = ((self.pc as i32) - 1 + offset) as usize,
      Nop(_) => {}
    }

    self.pc += 1
  }

  pub fn is_finished(&self) -> bool {
    self.pc >= self.instructions.len()
  }

  pub fn step(&mut self) -> bool {
    if self.is_finished() || self.has_executed[self.pc] {
      return false;
    }
    self.has_executed[self.pc] = true;
    self.execute_instruction();
    true
  }

  pub fn run(&mut self) -> i32 {
    loop {
      if self.step() == false {
        return self.acc;
      }
    }
  }

  pub fn reset(&mut self) {
    self.pc = 0;
    self.acc = 0;
    self.has_executed = vec![false; self.instructions.len()];
  }
}

fn main() -> Result<(), Box<dyn Error>> {
  let mut program = parse_input()?;
  let og_instructions = program.instructions.clone();

  let part_one = program.run();

  // brute force cba to make a graph and solve
  for (idx, ins) in og_instructions.iter().enumerate() {
    program.reset();
    match ins {
      Acc(_) => continue,
      Jmp(offset) => {
        program.instructions[idx] = Nop(*offset);
        program.run();
        if program.is_finished() {
          break;
        }
        program.instructions[idx] = Jmp(*offset);
      }
      Nop(offset) => {
        program.instructions[idx] = Jmp(*offset);
        program.run();
        if program.is_finished() {
          break;
        }
        program.instructions[idx] = Nop(*offset);
      }
    };
  }

  let part_two = program.acc;

  println!("{}", part_one);
  println!("{}", part_two);

  Ok(())
}

fn parse_input() -> Result<Program, std::io::Error> {
  let mut input = String::new();
  File::open("./input/problem")?.read_to_string(&mut input)?;

  let read = input
    .split(&format!("{}", LINE_ENDING))
    .map(|line| {
      let (lhs, rhs) = line.split(" ").collect_tuple().unwrap();
      match lhs {
        "acc" => Acc(rhs.parse::<i32>().unwrap()),
        "jmp" => Jmp(rhs.parse::<i32>().unwrap()),
        "nop" => Nop(rhs.parse::<i32>().unwrap()),
        unknown => panic!(format!("unknown instruction {}", unknown)),
      }
    })
    .collect_vec();

  let program = Program::new(read);
  Ok(program)
}
