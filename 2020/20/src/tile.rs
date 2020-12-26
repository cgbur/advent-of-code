use crate::tile::Rotation::*;

#[derive(Debug, Copy, Clone)]
pub enum Direction {
  Up,
  Left,
  Right,
  Down,
}
pub type Neighbor = (Direction, isize, isize);

pub const NEIGHBORS: [Neighbor; 4] = [
  (Direction::Left, -1, 0),
  (Direction::Up, 0, 1),
  (Direction::Down, 0, -1),
  (Direction::Right, 1, 0),
];

pub const ROTATIONS: [Rotation; 4] = [Normal, Left, Right, Over];
pub const FLIPPED: [bool; 2] = [true, false];

#[derive(Debug, Clone)]
pub enum Rotation {
  Normal,
  Left,
  Right,
  Over,
}

#[derive(Debug, Clone)]
pub struct Tile {
  pub id: usize,
  pub rotation: Rotation,
  pub flipped: bool,
  top: Vec<bool>,
  top_r: Vec<bool>,
  bottom: Vec<bool>,
  bottom_r: Vec<bool>,
  left: Vec<bool>,
  left_r: Vec<bool>,
  right: Vec<bool>,
  right_r: Vec<bool>,
}

impl Tile {
  pub fn new(id: usize, pixels: Vec<Vec<bool>>) -> Self {
    let top = pixels[0].clone();
    let bottom = pixels[pixels.len() - 1].clone();
    let mut left = vec![];
    let mut right = vec![];

    for y in 0..pixels.len() {
      left.push(pixels[y][0]);
      right.push(pixels[y][pixels[y].len() - 1])
    }

    let mut top_r = top.clone();
    top_r.reverse();

    let mut left_r = left.clone();
    left_r.reverse();

    let mut right_r = right.clone();
    right_r.reverse();

    let mut bottom_r = bottom.clone();
    bottom_r.reverse();

    Self {
      id,
      rotation: Normal,
      flipped: false,
      top,
      top_r,
      bottom,
      bottom_r,
      left,
      left_r,
      right,
      right_r,
    }
  }

  fn top_no_flip(&self) -> &[bool] {
    match &self.rotation {
      Normal => &self.top,
      Left => &self.right,
      Right => &self.left_r,
      Over => &self.bottom_r,
    }
  }

  fn left_no_flip(&self) -> &[bool] {
    match &self.rotation {
      Normal => &self.left,
      Left => &self.top_r,
      Right => &self.bottom,
      Over => &self.right_r,
    }
  }

  fn right_no_flip(&self) -> &[bool] {
    match &self.rotation {
      Normal => &self.right,
      Left => &self.bottom_r,
      Right => &self.top,
      Over => &self.left_r,
    }
  }

  fn bottom_no_flip(&self) -> &[bool] {
    match &self.rotation {
      Normal => &self.bottom,
      Left => &self.left,
      Right => &self.right_r,
      Over => &self.top_r,
    }
  }
  pub fn top(&self) -> &[bool] {
    if !self.flipped {
      self.top_no_flip()
    } else {
      self.bottom_no_flip()
    }
  }

  pub fn left(&self) -> &[bool] {
    if !self.flipped {
      self.left_no_flip()
    } else {
      self.right_no_flip()
    }
  }

  pub fn right(&self) -> &[bool] {
    if !self.flipped {
      self.right_no_flip()
    } else {
      self.left_no_flip()
    }
  }

  pub fn bottom(&self) -> &[bool] {
    if !self.flipped {
      self.bottom_no_flip()
    } else {
      self.top_no_flip()
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::tile::Rotation::{Left, Over, Right};
  use crate::tile::Tile;

  #[test]
  fn cons() {
    let mut tiles = vec![
      vec![false, false, false],
      vec![false, false, false],
      vec![true, false, false],
    ];

    let mut t = Tile::new(10, tiles);
    let mut t2 = t.clone();
    t2.rotation = Over;

    println!("{}", t.right() == t2.left());
    println!("{:?}", t2.left());
  }

  #[test]
  fn t() {
    let mut pixels = vec![
      vec![false, false, false],
      vec![false, false, false],
      vec![true, false, false],
    ];

    let mut tile = Tile::new(1, pixels);

    assert_eq!(tile.left(), vec![false, false, true]);
    tile.rotation = Left;
    assert_eq!(tile.bottom(), vec![false, false, true]);
    tile.rotation = Over;
    assert_eq!(tile.top(), vec![false, false, true]);
    tile.rotation = Right;
    assert_eq!(tile.right(), vec![false, false, false]);
  }
}
