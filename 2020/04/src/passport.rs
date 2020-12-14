use crate::LINE_ENDING;

const VALID_EYE_COLOR: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

fn is_hex(c: &str) -> bool {
  c.len() == 7 && c.starts_with("#") && u32::from_str_radix(&c[1..], 16).is_ok()
}

#[derive(Debug)]
pub struct Passport {
  byr: Option<u32>,
  iyr: Option<u32>,
  eyr: Option<u32>,
  hgt: Option<String>,
  hcl: Option<String>,
  ecl: Option<String>,
  pid: Option<String>,
  cid: Option<u32>,
}

impl Passport {
  pub fn from_line(line: &str) -> Self {
    let mut new_passport = Self {
      byr: None,
      iyr: None,
      eyr: None,
      hgt: None,
      hcl: None,
      ecl: None,
      pid: None,
      cid: None,
    };

    line.split(" ")
        .flat_map(|line| line.split(LINE_ENDING).collect::<Vec<_>>())
        .map(|item| item.split(":").collect::<Vec<_>>())
        .for_each(|kv| {
          let val = kv[1];
          match kv[0] {
            "byr" => new_passport.byr = Some(val.parse().unwrap()),
            "iyr" => new_passport.iyr = Some(val.parse().unwrap()),
            "eyr" => new_passport.eyr = Some(val.parse().unwrap()),
            "hgt" => new_passport.hgt = Some(val.parse().unwrap()),
            "hcl" => new_passport.hcl = Some(val.parse().unwrap()),
            "ecl" => new_passport.ecl = Some(val.parse().unwrap()),
            "pid" => new_passport.pid = Some(val.parse().unwrap()),
            "cid" => new_passport.cid = Some(val.parse().unwrap()),
            _ => {}
          }
        });
    new_passport
  }

  pub fn is_valid_part_one(&self) -> bool {
    self.byr.is_some() &&
        self.iyr.is_some() &&
        self.eyr.is_some() &&
        self.hgt.is_some() &&
        self.hcl.is_some() &&
        self.ecl.is_some() &&
        self.pid.is_some()
  }
  pub fn is_valid_part_two(&self) -> bool {
    if !self.is_valid_part_one() {
      return false;
    }

    if let Some(year) = self.byr {
      if !(1920 <= year && year <= 2002) {
        return false;
      }
    }

    if let Some(year) = self.iyr {
      if !(2010 <= year && year <= 2020) {
        return false;
      }
    }

    if let Some(year) = self.eyr {
      if !(2020 <= year && year <= 2030) {
        return false;
      }
    }

    if let Some(height) = &self.hgt {
      if !(height.contains("in") || height.contains("cm")) {
        return false;
      }

      let num = height.chars()
          .filter(|c| c.is_numeric())
          .collect::<String>()
          .parse::<u32>().unwrap();

      if height.contains("cm") {
        if !(150 <= num && num <= 193) {
          return false;
        }
      } else {
        if !(59 <= num && num <= 76) {
          return false;
        }
      }
    }

    if let Some(color) = &self.hcl {
      if !is_hex(color) {
        return false;
      }
    }

    if let Some(color) = &self.ecl {
      if !VALID_EYE_COLOR.contains(&&**color) {
        return false;
      }
    }

    if let Some(pid) = &self.pid {
      if !(pid.len() == 9 &&
          pid.chars().map(|c| c.is_numeric() as u32).sum::<u32>() == 9) {
        return false;
      }
    }

    return true;
  }
}


