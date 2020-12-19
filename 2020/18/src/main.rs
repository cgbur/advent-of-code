use std::error::Error;
use std::fs::File;
use std::io::Read;

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

fn char_to_u64(c: char) -> u64 {
  c.to_digit(10).unwrap() as u64
}


fn add_precedence(mut line: String) -> String {
  while let Some(pos) = line.chars().position(|c| c == '.') {
    line.replace_range(pos..pos + 1, "+");
    let mut balance = 0;
    let mut left = pos - 1;
    loop {
      match line.chars().nth(left).unwrap() {
        ')' => balance += 1,
        '(' => balance -= 1,
        '0'..='9' => {
          if balance == 0 {
            line.insert(left, '(');
            break;
          }
        }
        _ => {}
      };
      if balance == 0 {
        line.insert(left, '(');
        break;
      }
      left -= 1;
    }

    let mut right = pos + 2;

    loop {
      match line.chars().nth(right).unwrap() {
        '(' => balance += 1,
        ')' => balance -= 1,
        '0'..='9' => {
          if balance == 0 {
            line.insert(right + 1, ')');
            break;
          }
        }
        _ => {}
      };
      if balance == 0 {
        line.insert(right, ')');
        break;
      }
      right += 1;
    }
  }

  line
}

fn eval(line: String) -> u64 {
  let mut op = '+';
  let mut ans = 0;
  let mut skip = 0;
  for (index, char) in line.chars().enumerate() {
    if skip > 0 {
      skip -= 1;
      continue;
    }
    match char {
      '0'..='9' => {
        match op {
          '+' => ans += char_to_u64(char),
          '*' => ans *= char_to_u64(char),
          _ => panic!("unkown op")
        };
      }
      '+' | '*' => op = char,
      '(' => {
        let mut balance = 1;
        let mut end = 0;
        for (idx, c) in (&line[index + 1..]).chars().enumerate() {
          match c {
            '(' => balance += 1,
            ')' => balance -= 1,
            _ => {}
          }
          if balance == 0 {
            end = idx;
            skip = idx + 1;
            break;
          }
        };
        let res = eval(line[index + 1..index + end + 1].to_string());
        match op {
          '+' => ans += res,
          '*' => ans *= res,
          _ => panic!("unkown op")
        };
      }
      ')' => return ans,
      _ => panic!("bad char in input")
    }
  }
  ans
}

fn main() -> Result<(), Box<dyn Error>> {
  let mut input = String::new();
  File::open("./input/problem")?.read_to_string(&mut input)?;

  let lines = input.split(LINE_ENDING)
      .map(|line| line.replace(" ", ""))
      .filter(|line| line.len() > 0)
      .map(|line| (eval(line.clone()), eval(add_precedence(line.replace("+", ".")))))
      .fold((0, 0), |(p1_acc, p2_acc), (p1_val, p2_val)| {
        (p1_acc + p1_val, p2_acc + p2_val)
      });

  println!("{:?}", lines);
  
  Ok(())
}
