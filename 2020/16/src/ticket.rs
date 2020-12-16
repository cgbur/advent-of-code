use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct Ticket {
  pub values: Vec<u32>,
}

impl Ticket {
  pub fn from_line(line: &str) -> Self {
    Self {
      values: line
        .split(",")
        .map(|s| s.parse::<u32>().unwrap())
        .collect_vec(),
    }
  }
}
