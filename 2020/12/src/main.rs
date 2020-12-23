use itertools::Itertools;
use std::error::Error;
use std::fs::File;
use std::io::Read;

#[cfg(windows)]
const LINE_ENDING: &str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &str = "\n";

/// Overkill because all rotations are 0 90 180 etc.
/// optimal would be to write a function that maps num/90 to get
/// the operation that would flip x and y to get the new coord.
/// Example (x,y) = (y, -x) for  a rotation
///
/// Instead here is rotate matrix so you can accept any amount of
/// rotation.
///
/// ```ignore
/// |cos -sin| |x|
/// |sin cos|  |y|
///
/// x' = cos(t) * x - sin(x) * y
/// y' = sin(x) * x + cos(x) * y
/// ```
fn rotate_point((x, y): (isize, isize), theta: isize) -> (isize, isize) {
  let (x, y) = (x as f32, y as f32);
  let theta = theta as f32;
  let cos_theta = theta.to_radians().cos();
  let sin_theta = theta.to_radians().sin();

  (
    (cos_theta * x - sin_theta * y).round() as isize,
    (sin_theta * x + cos_theta * y).round() as isize,
  )
}

fn main() -> Result<(), Box<dyn Error>> {
  let inputs = parse_input()?;

  let (p1x, p1y, _) = inputs
    .iter()
    .fold((0, 0, 0), |(mut x, mut y, mut theta), &(dir, dist)| {
      match dir {
        'N' => y += dist,
        'S' => y -= dist,
        'E' => x += dist,
        'W' => x -= dist,
        'L' => theta += dist,
        'R' => theta -= dist,
        'F' => {
          x += ((theta as f32).to_radians().cos() * dist as f32) as isize;
          y += ((theta as f32).to_radians().sin() * dist as f32) as isize;
        }
        _ => panic!("unknown dir found"),
      }
      (x, y, theta)
    });

  let (_, _, p2x, p2y) = inputs.iter().fold(
    (10, 1, 0, 0),
    |(mut x, mut y, mut boat_x, mut boat_y), &(dir, dist)| {
      match dir {
        'N' => y += dist,
        'S' => y -= dist,
        'E' => x += dist,
        'W' => x -= dist,
        'L' => {
          let (nx, ny) = rotate_point((x, y), dist);
          x = nx;
          y = ny;
        }
        'R' => {
          let (nx, ny) = rotate_point((x, y), -dist);
          x = nx;
          y = ny;
        }
        'F' => {
          boat_x += x * dist;
          boat_y += y * dist;
        }
        _ => panic!("unknown dir found"),
      }
      (x, y, boat_x, boat_y)
    },
  );

  println!("{:?}", p1x.abs() + p1y.abs());
  println!("{:?}", p2x.abs() + p2y.abs());

  Ok(())
}

fn parse_input() -> Result<Vec<(char, isize)>, std::io::Error> {
  let mut input = String::new();
  File::open("./input/problem")?.read_to_string(&mut input)?;

  let read = input
    .split(LINE_ENDING)
    .map(|line| {
      let dir = (&line[0..1]).chars().next().unwrap();
      let dist = (&line[1..]).parse::<isize>().unwrap();

      (dir, dist)
    })
    .collect_vec();

  Ok(read)
}
