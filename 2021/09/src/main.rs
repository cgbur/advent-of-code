use itertools::Itertools;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    File::open("./input")?.read_to_string(&mut input)?;

    let map = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec();

    let low_points = (0..map[0].len())
        .cartesian_product(0..map.len())
        .filter(|&(row, col)| {
            neighbor_coord(row, col, &map)
                .iter()
                .all(|&(x, y)| map[y][x] > map[col][row])
        });

    let part_one = low_points
        .clone()
        .map(|(row, col)| map[col][row] + 1)
        .sum::<u32>();

    let part_two = low_points
        .map(|(x, y)| basin(x, y, &map))
        .sorted()
        .rev()
        .take(3)
        .product::<usize>();

    println!("{}", part_one);
    println!("{}", part_two);

    Ok(())
}

fn neighbor_coord(x: usize, y: usize, map: &[Vec<u32>]) -> Vec<(usize, usize)> {
    neighbors(x, y)
        .into_iter()
        .filter(|&(x, y)| map.get(y).and_then(|f| f.get(x)).is_some())
        .collect_vec()
}

fn neighbors(x: usize, y: usize) -> [(usize, usize); 4] {
    [
        (x + 1, y),
        (x.wrapping_sub(1), y),
        (x, y + 1),
        (x, y.wrapping_sub(1)),
    ]
}

fn basin(x: usize, y: usize, map: &[Vec<u32>]) -> usize {
    let mut visited = HashSet::new();
    let mut next = vec![];
    let mut size = 0;

    next.push((x, y));

    while let Some((x, y)) = next.pop() {
        if visited.contains(&(x, y)) {
            continue;
        }

        visited.insert((x, y));
        size += 1;

        neighbor_coord(x, y, map)
            .iter()
            .filter(|&&(nx, ny)| map[ny][nx] != 9)
            .filter(|&&(nx, ny)| !visited.contains(&(nx, ny)))
            .filter(|&&(nx, ny)| map[ny][nx] >= map[y][x])
            .for_each(|n| next.push(*n));
    }

    size
}
