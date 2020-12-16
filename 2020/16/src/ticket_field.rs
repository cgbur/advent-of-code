use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct TicketField {
  pub identifier: String,
  pub lower_range: (u32, u32),
  pub upper_range: (u32, u32),
  pub index: Option<u32>,
  pub possible_indices: Vec<usize>,
}

fn range_from_str(line: &str) -> (u32, u32) {
  let r = line
    .split("-")
    .map(|n| n.trim().parse::<u32>().unwrap())
    .collect_vec();
  (r[0], r[1])
}

fn in_range(num: u32, range: (u32, u32)) -> bool {
  range.0 <= num && num <= range.1
}

impl TicketField {
  pub fn from_line(line: &str) -> Self {
    let pre_split = line.split(":").collect_vec();
    let matches = pre_split[1].split(" ").collect_vec();
    let identifier = pre_split[0][0..pre_split[0].len()].to_string();

    let lower_range = range_from_str(&matches[1]);
    let upper_range = range_from_str(&matches[3]);

    Self {
      identifier,
      lower_range,
      upper_range,
      index: None,
      possible_indices: vec![],
    }
  }

  pub fn is_valid(&self, field: u32) -> bool {
    self.in_lower_range(field) || self.in_upper_range(field)
  }

  fn in_lower_range(&self, num: u32) -> bool {
    in_range(num, self.lower_range)
  }

  fn in_upper_range(&self, num: u32) -> bool {
    in_range(num, self.upper_range)
  }
}
