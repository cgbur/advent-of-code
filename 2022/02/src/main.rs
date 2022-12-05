use aoc::input;
use itertools::Itertools;

fn main() {
    let score_1 = input()
        .lines()
        .map(|elf| {
            let (op, me) = elf.split(" ").collect_tuple().unwrap();
            rock_paper_sciccors(op, me)
        })
        .sum::<u32>();

    let score_2 = input()
        .lines()
        .map(|elf| {
            let (op, me) = elf.split(" ").collect_tuple().unwrap();
            rock_paper_sciccors_2(op, me)
        })
        .sum::<u32>();

    println!("Part 1: {}", score_1);
    println!("Part 2: {}", score_2);
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Winner {
    Me,
    Opponent,
    Tie,
}

fn rock_paper_sciccors_2(opponent: &str, me: &str) -> u32 {
    // A rock B is paper C is sciccors
    // right is X rock B is paper C is sciccors
    let op = match opponent {
        "A" => Choice::Rock,
        "B" => Choice::Paper,
        "C" => Choice::Scissors,
        _ => panic!("Invalid input"),
    };

    let me = match me {
        "X" => Winner::Opponent,
        "Y" => Winner::Tie,
        "Z" => Winner::Me,
        _ => panic!("Invalid input"),
    };

    let to_play_to_fix_outcome = match op {
        Choice::Rock => match me {
            Winner::Opponent => Choice::Scissors,
            Winner::Tie => Choice::Rock,
            Winner::Me => Choice::Paper,
        },
        Choice::Paper => match me {
            Winner::Opponent => Choice::Rock,
            Winner::Tie => Choice::Paper,
            Winner::Me => Choice::Scissors,
        },
        Choice::Scissors => match me {
            Winner::Opponent => Choice::Paper,
            Winner::Tie => Choice::Scissors,
            Winner::Me => Choice::Rock,
        },
    };

    let score = match me {
        Winner::Me => 6,
        Winner::Opponent => 0,
        Winner::Tie => 3,
    } + match to_play_to_fix_outcome {
        Choice::Rock => 1,
        Choice::Paper => 2,
        Choice::Scissors => 3,
    };

    score
}

fn rock_paper_sciccors(opponent: &str, me: &str) -> u32 {
    // A rock B is paper C is sciccors
    // right is X rock B is paper C is sciccors
    let op = match opponent {
        "A" => Choice::Rock,
        "B" => Choice::Paper,
        "C" => Choice::Scissors,
        _ => panic!("Invalid input"),
    };
    let me = match me {
        "X" => Choice::Rock,
        "Y" => Choice::Paper,
        "Z" => Choice::Scissors,
        _ => panic!("Invalid input"),
    };

    let winner = match (me, op) {
        (Choice::Rock, Choice::Paper) => Winner::Opponent,
        (Choice::Rock, Choice::Scissors) => Winner::Me,
        (Choice::Paper, Choice::Rock) => Winner::Me,
        (Choice::Paper, Choice::Scissors) => Winner::Opponent,
        (Choice::Scissors, Choice::Rock) => Winner::Opponent,
        (Choice::Scissors, Choice::Paper) => Winner::Me,
        _ => Winner::Tie,
    };

    let score = match winner {
        Winner::Me => 6,
        Winner::Opponent => 0,
        Winner::Tie => 3,
    } + match me {
        Choice::Rock => 1,
        Choice::Paper => 2,
        Choice::Scissors => 3,
    };

    score
}
