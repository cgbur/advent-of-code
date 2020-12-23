use crate::Seat::{Floor, Open, Taken};
use itertools::Itertools;
use std::error::Error;
use std::fs::File;
use std::io::Read;

#[cfg(windows)]
const LINE_ENDING: &str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &str = "\n";

type OccupiedMapper = Box<dyn Fn(&[Vec<Seat>]) -> Vec<Vec<i32>>>;

const NEIGHBORS: [(isize, isize); 8] = [
  (-1, -1),
  (-1, 0),
  (-1, 1),
  (0, -1),
  (0, 1),
  (1, -1),
  (1, 0),
  (1, 1),
];

#[derive(Debug, Copy, Clone)]
enum Seat {
  Open,
  Floor,
  Taken,
}

fn occupied_chooser(version: &str) -> OccupiedMapper {
  let occupied_searcher = match version {
    "p1" => occupied_p1,
    "p2" => occupied_p2,
    &_ => panic!("unknown occupied version"),
  };

  Box::new(move |seats: &[Vec<Seat>]| -> Vec<Vec<i32>> {
    let height = seats.len();
    let width = seats[0].len();

    let mut occupied = vec![vec![0; width]; height];

    for y in 0..height {
      for x in 0..width {
        if let Taken = seats[y][x] {
          occupied_searcher(seats, &mut occupied, x, y, width, height);
        }
      }
    }

    occupied
  })
}

fn occupied_p1(
  _: &[Vec<Seat>],
  occupied: &mut Vec<Vec<i32>>,
  x: usize,
  y: usize,
  width: usize,
  height: usize,
) {
  NEIGHBORS
    .iter()
    .map(|(n_x, n_y)| (x as isize + n_x, y as isize + n_y))
    .filter(|&(x, y)| !(x < 0 || y < 0 || x >= width as isize || y >= height as isize))
    .map(|(x, y)| (x as usize, y as usize))
    .for_each(|(x, y)| occupied[y][x] += 1);
}

fn occupied_p2(
  seats: &[Vec<Seat>],
  occupied: &mut Vec<Vec<i32>>,
  x: usize,
  y: usize,
  width: usize,
  height: usize,
) {
  for (dx, dy) in &NEIGHBORS {
    let (mut sx, mut sy) = (x as isize, y as isize);
    sx += dx;
    sy += dy;

    while !(sx < 0 || sy < 0 || sx >= width as isize || sy >= height as isize) {
      let (nx, ny) = (sx as usize, sy as usize);
      match seats[ny][nx] {
        Floor => {}
        _ => {
          occupied[ny][nx] += 1;
          break;
        }
      };
      sx += dx;
      sy += dy;
    }
  }
}

fn seat_sim(mut seats: Vec<Vec<Seat>>, tolerance: i32, occupied_fn: OccupiedMapper) -> usize {
  let height = seats.len();
  let width = seats[0].len();

  loop {
    let occupied = occupied_fn(&seats);
    let mut no_change = true;
    for y in 0..height {
      for x in 0..width {
        match seats[y][x] {
          Floor => {}
          Open => {
            if occupied[y][x] == 0 {
              seats[y][x] = Taken;
              no_change = false;
            }
          }
          Taken => {
            if occupied[y][x] >= tolerance {
              seats[y][x] = Open;
              no_change = false;
            }
          }
        }
      }
    }

    if no_change {
      break;
    }
  }

  seats
    .iter()
    .flatten()
    .filter(|&seat| matches!(seat, Taken))
    .count()
}

fn main() -> Result<(), Box<dyn Error>> {
  let seats = parse_input()?;

  let part_one = seat_sim(seats.clone(), 4, occupied_chooser("p1"));
  let part_two = seat_sim(seats, 5, occupied_chooser("p2"));

  println!("{:?}", part_one);
  println!("{:?}", part_two);

  Ok(())
}

fn parse_input() -> Result<Vec<Vec<Seat>>, std::io::Error> {
  let mut input = String::new();
  File::open("./input/problem")?.read_to_string(&mut input)?;

  let read = input
    .split(&format!("{}", LINE_ENDING))
    .map(|line| {
      line
        .chars()
        .map(|char| match char {
          'L' => Open,
          '.' => Floor,
          '#' => Taken,
          _ => panic!("unknown seat choice"),
        })
        .collect_vec()
    })
    .collect_vec();

  Ok(read)
}
