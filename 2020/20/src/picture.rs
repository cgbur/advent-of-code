use crate::tile::{Direction, Rotation, Tile, FLIPPED, NEIGHBORS, ROTATIONS};
use itertools::{all, Itertools};
use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub struct State {
  choices: Vec<Tile>,
}

pub struct Picture {
  pub possible: Vec<Tile>,
  pub positions: Vec<Vec<Option<Tile>>>,
  stack: Vec<State>,
  size: usize,
}

impl Picture {
  pub fn new(mut possible: Vec<Tile>) -> Self {
    let size = (possible.len() as f32).sqrt() as usize;
    let mut positions = vec![];
    for y in 0..size {
      positions.push(vec![]);
      for _ in 0..size {
        positions[y].push(None);
      }
    }

    let mut all_tiles = vec![];
    for ((tile, rotation), flipped) in possible
      .iter()
      .cartesian_product(&ROTATIONS)
      .cartesian_product(&FLIPPED)
      .into_iter()
    {
      let mut t = tile.clone();
      t.rotation = rotation.clone();
      t.flipped = *flipped;
      all_tiles.push(t);
    }

    let choices = all_tiles.clone();
    let stack_item = State { choices };
    let stack = vec![stack_item];
    Self {
      possible: all_tiles,
      positions,
      stack,
      size,
    }
  }

  pub fn match_make(&mut self) {
    while !self.is_done() {
      let next_spot = self.first_free().unwrap();
      if let Some(choice) = self.get_possible(next_spot) {
        // alter state and push new one
        let choices = &mut self.current_state_mut().choices;
        let index = choices
          .iter()
          .position(|p| {
            let r1 = p.rotation.clone();
            let r2 = choice.rotation.clone();
            p.id == choice.id && matches!(r1, r2) && p.flipped == choice.flipped
          })
          .unwrap();
        choices.remove(index);
        let new_state = State {
          choices: choices.clone(),
        };

        self.stack.push(new_state);
      } else {
        // pop
      }
    }
  }

  pub fn current_state(&self) -> &State {
    self.stack.last().unwrap()
  }
  pub fn current_state_mut(&mut self) -> &mut State {
    self.stack.last_mut().unwrap()
  }

  fn get_isize(&self, (x, y): (isize, isize)) -> Option<Tile> {
    self.get((x as usize, y as usize))
  }

  fn get(&self, (x, y): (usize, usize)) -> Option<Tile> {
    if x >= self.size || y >= self.size {
      return None;
    }
    self.positions[y][x].clone()
  }

  fn set(&mut self, (x, y): (usize, usize), tile: Tile) {
    if x >= self.size || y >= self.size {
      return;
    }
    self.positions[y][x] = Some(tile);
  }

  fn first_free(&self) -> Option<(usize, usize)> {
    for y in 0..self.size {
      for x in 0..self.size {
        if self.positions[y][x].is_none() {
          return Some((x, y));
        }
      }
    }
    None
  }

  fn is_done(&self) -> bool {
    self.positions.iter().flatten().all(|p| p.is_some())
  }

  fn get_possible(&self, (x, y): (usize, usize)) -> Option<&Tile> {
    if self.possible.is_empty() {
      return None;
    }
    let restrictive_neighbors = NEIGHBORS
      .iter()
      .map(|&(dir, dx, dy)| (dir, self.get_isize((x as isize + dx, y as isize + dy))))
      .filter(|(dir, tile)| tile.is_some())
      .map(|(dir, tile)| (dir, tile.unwrap()))
      .collect_vec();

    if restrictive_neighbors.is_empty() {
      Some(self.current_state().choices.first().unwrap())
    } else {
      // self
      //   .current_state()
      //   .choices
      //   .iter()
      //   .filter(|&&p| {
      //     restrictive_neighbors
      //       .iter()
      //       .all(|(dir, neighbor)| match dir {
      //         Direction::Up => p.top() == neighbor.bottom(),
      //         Direction::Left => p.left() == neighbor.right(),
      //         Direction::Right => p.right() == neighbor.left(),
      //         Direction::Down => p.bottom() == neighbor.top(),
      //       })
      //   })
      //   .collect_vec()
      //   .first()
      None
    }
  }
}
