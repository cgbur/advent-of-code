use crate::picture::Picture;
use crate::tile::Rotation::Left;
use crate::tile::Tile;
use itertools::{max, Itertools};
use regex::Regex;
use std::error::Error;
use std::fs::File;
use std::io::Read;

#[cfg(windows)]
const LINE_ENDING: &str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &str = "\n";

mod picture;
mod tile;

fn main() -> Result<(), Box<dyn Error>> {
  let mut tiles = parse_input()?;

  let mut picture = Picture::new(tiles.clone());
  picture.match_make();

  // picture.print();

  // let part_one = picture
  //   .corners()
  //   .map(|(&x, y)| picture.positions[&(x, y)].id)
  //   .product::<usize>();
  //
  // println!("{}", part_one);

  Ok(())
}

fn parse_input() -> Result<Vec<Tile>, std::io::Error> {
  let mut input = String::new();
  File::open("./input/problem")?.read_to_string(&mut input)?;

  let numbers = "[0-9]{4}";
  let num_re = Regex::new(numbers).unwrap();

  let read = input
    .split(&format!("{}{}", LINE_ENDING, LINE_ENDING))
    .map(|tile| {
      let mut tile_iter = tile.split(LINE_ENDING);

      let id = num_re
        .captures(tile_iter.next().unwrap())
        .unwrap()
        .get(0)
        .unwrap()
        .as_str()
        .parse::<usize>()
        .unwrap();

      let pixels = tile_iter
        .map(|line| line.chars().map(|c| c == '#').collect_vec())
        .collect_vec();

      Tile::new(id, pixels)
    })
    .collect_vec();

  Ok(read)
}
