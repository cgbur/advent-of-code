use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Read;

#[cfg(windows)]
const LINE_ENDING: &str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &str = "\n";

// re-worked to be cleaner off of AxlLind's solution

#[derive(Clone, Debug)]
enum Rule {
  Comb(Vec<Vec<usize>>),
  Char(char),
}

fn as_regex(rules: &HashMap<usize, Rule>, id: usize) -> String {
  match &rules[&id] {
    Rule::Char(c) => c.to_string(),
    Rule::Comb(v) => {
      let re = v
        .iter()
        .map(|p| p.iter().map(|&id| as_regex(rules, id)).join(""))
        .join("|");
      format!("({})", re)
    }
  }
}

fn part_one(rules: &HashMap<usize, Rule>, input: &[String]) -> usize {
  let re_pat = as_regex(rules, 0);
  let re = Regex::new(&format!("^{}$", re_pat)).unwrap();
  input.iter().filter(|s| re.is_match(s)).count()
}

fn part_two(rules: &HashMap<usize, Rule>, input: &[String]) -> usize {
  let re_31 = as_regex(&rules, 31);
  let re_42 = as_regex(&rules, 42);
  let rule_new = (1..6)
    .map(|i| format!("{r1}{{{n}}}{r2}{{{n}}}", r1 = re_42, r2 = re_31, n = i))
    .join("|");
  let re_str = format!("^{}+({})$", re_42, rule_new);
  let re = Regex::new(&re_str).unwrap();
  input.iter().filter(|s| re.is_match(s)).count()
}

fn main() -> Result<(), Box<dyn Error>> {
  let (rules, input) = parse_input()?;

  let part_one = part_one(&rules, &input);
  let part_two = part_two(&rules, &input);

  println!("{}", part_one);
  println!("{}", part_two);

  Ok(())
}

fn parse_input() -> Result<(HashMap<usize, Rule>, Vec<String>), std::io::Error> {
  let mut input = String::new();
  File::open("./input/problem")?.read_to_string(&mut input)?;

  let mut read = input
    .split(&format!("{}{}", LINE_ENDING, LINE_ENDING))
    .collect_vec();

  let rules = read[0]
    .split(LINE_ENDING)
    .flat_map(|s| s.split(": "))
    .tuples()
    .map(|(id, s)| {
      let id = id.parse::<usize>().unwrap();
      if s.contains('"') {
        return (id, Rule::Char(s.as_bytes()[1] as char));
      }
      let v = s
        .split(" | ")
        .map(|p| {
          p.split_whitespace()
            .map(|i| i.parse::<usize>().unwrap())
            .collect()
        })
        .collect();
      (id, Rule::Comb(v))
    })
    .collect::<HashMap<usize, Rule>>();

  let input_lines = read[1]
    .split(LINE_ENDING)
    .map(|s| s.to_string())
    .collect_vec();
  Ok((rules, input_lines))
}
