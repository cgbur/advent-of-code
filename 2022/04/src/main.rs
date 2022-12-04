use aoc::input;
use std::str::FromStr;

fn main() {
    let part_one = input()
        .lines()
        .filter(|line| {
            let WorkOrder(left, right) = line.parse().unwrap();
            left.contains(right) || right.contains(left)
        })
        .count();

    let part_two = input()
        .lines()
        .filter(|line| {
            let WorkOrder(left, right) = line.parse().unwrap();
            left.overlaps(right)
        })
        .count();

    println!("{:?}", part_one);
    println!("{:?}", part_two);
}

#[derive(Debug, Clone, Copy)]
struct Range {
    start: u32,
    stop: u32,
}

impl Range {
    fn overlaps(&self, other: Range) -> bool {
        self.start <= other.stop && self.stop >= other.start
    }

    fn contains(&self, other: Range) -> bool {
        self.start <= other.start && self.stop >= other.stop
    }
}

impl FromStr for Range {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, stop) = s.split_once('-').unwrap();
        Ok(Range {
            start: start.parse().unwrap(),
            stop: stop.parse().unwrap(),
        })
    }
}

struct WorkOrder(Range, Range);

impl FromStr for WorkOrder {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(',').unwrap();
        Ok(WorkOrder(left.parse().unwrap(), right.parse().unwrap()))
    }
}
