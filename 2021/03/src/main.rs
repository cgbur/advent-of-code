use advent_util::LINE_ENDING;
use itertools::Itertools;
use std::error::Error;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Clone)]
struct Board {
    numbers: Vec<Vec<u32>>,
    hits: Vec<Vec<bool>>,
    size: usize,
    disabled: bool,
}

impl Board {
    pub fn new(board: Vec<Vec<u32>>) -> Self {
        let hits = board
            .iter()
            .map(|row| row.iter().map(|_num| false).collect_vec())
            .collect_vec();

        Self {
            size: board.len(),
            numbers: board,
            hits,
            disabled: false,
        }
    }

    pub fn play(&mut self, num: u32) {
        for y in 0..self.size {
            for x in 0..self.size {
                if self.numbers[y][x] == num {
                    self.hits[y][x] = true;
                }
            }
        }
    }

    pub fn disable(&mut self) {
        self.disabled = true;
    }

    pub fn won(&self) -> bool {
        !self.disabled && (self.won_horizontal() || self.won_vertical())
    }

    fn won_vertical(&self) -> bool {
        for y in 0..self.size {
            let mut won = true;
            for x in 0..self.size {
                if self.hits[y][x] == false {
                    won = false;
                    break;
                }
            }
            if won {
                return won;
            }
        }
        false
    }

    fn won_horizontal(&self) -> bool {
        for x in 0..self.size {
            let mut won = true;
            for y in 0..self.size {
                if self.hits[y][x] == false {
                    won = false;

                    break;
                }
            }
            if won {
                return won;
            }
        }
        false
    }

    pub fn score(&self) -> u32 {
        self.numbers
            .iter()
            .flatten()
            .zip(self.hits.iter().flatten())
            .filter_map(|(num, hit)| if !hit { Some(num) } else { None })
            .sum()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let (numbers, boards) = parse_input()?;

    println!("{}", part_one(&numbers.clone(), &mut boards.clone()));
    println!("{}", part_two(&numbers, boards));
    Ok(())
}

fn part_one(numbers: &[u32], boards: &mut [Board]) -> u32 {
    for &number in numbers {
        boards.iter_mut().for_each(|board| board.play(number));
        if let Some(board) = boards.iter().find(|board| board.won()) {
            return number * board.score();
        }
    }

    panic!("Nobody wins???")
}

fn part_two(numbers: &[u32], mut boards: Vec<Board>) -> u32 {
    let mut last_score = 0;

    for &number in numbers {
        boards.iter_mut().for_each(|board| board.play(number));
        for board in boards.iter_mut().filter(|board| board.won()) {
            board.disable();
            last_score = number * board.score();
        }
    }

    last_score
}

fn parse_input() -> Result<(Vec<u32>, Vec<Board>), Box<dyn Error>> {
    let mut input = String::new();
    File::open("./input")?.read_to_string(&mut input)?;

    let splits = input
        .split(&format!("{}{}", LINE_ENDING, LINE_ENDING))
        .collect_vec();

    let mut splits_iter = splits.iter();

    let numbers = splits_iter
        .next()
        .unwrap()
        .split(",")
        .map(|num| num.parse::<u32>().unwrap())
        .collect_vec();

    let boards = splits_iter
        .map(|num| {
            num.lines()
                .map(|line| {
                    line.split_whitespace()
                        .map(|c| c.parse::<u32>().unwrap())
                        .collect_vec()
                })
                .collect_vec()
        })
        .map(|board| Board::new(board))
        .collect_vec();

    Ok((numbers, boards))
}
