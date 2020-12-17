use itertools::{Itertools, MultiProduct};
use std::collections::HashMap;
use std::convert::TryInto;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::ops::{RangeInclusive};


#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

// dimensions >= 2
const DIMENSIONS: usize = 4;
const ROUNDS: usize = 6;

type Dimension = [isize; DIMENSIONS];
type Activity = (bool, bool);

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Cube {
  dim: Dimension,
}

fn gen_cartesian(num_dimensions: usize, low: isize, high: isize)
                 -> MultiProduct<RangeInclusive<isize>> {
  (0..num_dimensions).map(|_i| low..=high).multi_cartesian_product()
}

fn get_neighbors(cube: &Cube) -> Vec<Cube> {
  let mut cubes = vec![];

  for dim in gen_cartesian(DIMENSIONS, -1, 1) {
    if !dim.iter().all(|&n| n == 0) {
      let located = dim.iter()
          .zip(cube.dim.iter())
          .map(|(local, cube)| cube + local)
          .collect_vec();

      cubes.push(Cube {
        dim: located.try_into().unwrap_or_else(|_| panic!("unknown number of dimensions")),
      })
    }
  }

  cubes
}

fn main() -> Result<(), Box<dyn Error>> {
  let mut state = parse_input()?;

  for _ in 0..ROUNDS {
    let current_cubes = state.keys().cloned().collect_vec();

    for cube in &current_cubes {
      let neighbors = get_neighbors(&cube);
      let mut active_count = 0;

      for neighbor in neighbors {
        let (active, _) = state.entry(neighbor).or_insert((false, false));
        if *active { active_count += 1 }
        if active_count > 3 { break; }
      }

      let (is_current_active, next_active) =
          state.entry(cube.clone()).or_default();

      if *is_current_active {
        *next_active = active_count == 2 || active_count == 3;
      } else {
        *next_active = active_count == 3;
      }
    }

    for cube in current_cubes {
      let (active, next_active) = state.entry(cube.clone()).or_default();
      *active = *next_active;
      *next_active = false;
    }
  }

  let num_active = state
      .values()
      .map(|&(active, _)| active as u32)
      .sum::<u32>();

  println!("{}", num_active);

  Ok(())
}

fn parse_input() -> Result<HashMap<Cube, Activity>, std::io::Error> {
  let mut input = String::new();
  File::open("./input/problem")?.read_to_string(&mut input)?;

  let active_map = input
      .split(LINE_ENDING)
      .map(|line| line.chars().map(|e| e == '#').collect_vec())
      .collect_vec();

  let dim_len = active_map.len();

  let mut state = gen_cartesian(2, 0, (dim_len - 1) as isize).map(|dim| {
    let x = dim[0];
    let y = dim[1];

    let mut dim = [0; DIMENSIONS];
    dim[0] = x;
    dim[1] = y;

    (Cube { dim }, (active_map[y as usize][x as usize], false))
  }).collect::<HashMap<_, _>>();


  let current_cubes = state.keys().cloned().collect_vec();

  for cube in current_cubes {
    let neighbors = get_neighbors(&cube);
    for neighbor in neighbors {
      state.entry(neighbor).or_insert((false, false));
    }
  }

  Ok(state)
}