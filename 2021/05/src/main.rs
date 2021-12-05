use itertools::Itertools;

use std::collections::{HashMap};
use std::error::Error;
use std::fs::File;
use std::io::Read;


#[derive(Clone, Copy)]
struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let lines = parse_input()?;

    let hor_and_vert = lines
        .iter()
        .filter(|(start, end)| start.x == end.x || start.y == end.y)
        .copied()
        .collect_vec();

    println!("{}", num_lines(&hor_and_vert));
    println!("{}", num_lines(&lines));

    Ok(())
}

fn num_lines(lines: &[(Point, Point)]) -> usize {
    let mut map = HashMap::new();

    for (start, end) in lines {
        let dx = (end.x - start.x).signum();
        let dy = (end.y - start.y).signum();

        let (mut x, mut y) = (start.x, start.y);

        while (x, y) != (end.x + dx, end.y + dy) {
            *map.entry((x, y)).or_insert(0) += 1;
            x += dx;
            y += dy;
        }
    }

    map.values().filter(|p| **p > 1).count()
}

fn parse_input() -> Result<Vec<(Point, Point)>, std::io::Error> {
    let mut input = String::new();
    File::open("./input")?.read_to_string(&mut input)?;

    let mut lines = Vec::new();
    input.lines().for_each(|line| {
        let splits = line.split(" -> ").collect_vec();
        let (x1, y1) = splits[0]
            .split(',')
            .map(|s| s.parse::<i32>().unwrap())
            .collect_tuple()
            .unwrap();
        let (x2, y2) = splits[1]
            .split(',')
            .map(|s| s.parse::<i32>().unwrap())
            .collect_tuple()
            .unwrap();

        lines.push((Point::new(x1, y1), Point::new(x2, y2)));
    });

    Ok(lines)
}
