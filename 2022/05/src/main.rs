use aoc::{input, Aoc};
use itertools::Itertools;

fn main() {
    let mut inputs = input().groups();
    let mut stacks = vec![vec![]; 9];
    let state = inputs.next().unwrap();
    let commands = inputs.next().unwrap();

    state.lines().take(8).for_each(|line| {
        (0..9)
            .map(|pos| (pos, 4 * pos + 1))
            .for_each(|(stack, pos)| {
                let c = line.chars().nth(pos).unwrap();
                if c != ' ' {
                    stacks[stack].push(c);
                }
            });
    });

    for stack in &mut stacks {
        stack.reverse();
    }

    let stacks_clone = stacks.clone();
    for command in commands.lines() {
        let (count, from, to) = command
            .split_whitespace()
            .filter_map(|s| s.parse::<usize>().ok())
            .collect_tuple()
            .unwrap();
        for _ in 0..count {
            let block = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(block);
        }
    }

    let part_one = stacks.iter().map(|s| s.last().unwrap()).join("");
    println!("Part one: {}", part_one);

    let mut stacks = stacks_clone;
    for command in commands.lines() {
        let (count, from, to) = command
            .split_whitespace()
            .filter_map(|s| s.parse::<usize>().ok())
            .collect_tuple()
            .unwrap();
        for _ in 0..count {
            let block = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(block);
        }
        let stack_len = stacks[to - 1].len();
        stacks[to - 1][stack_len - count..].reverse();
    }

    let part_two = stacks.iter().map(|s| s.last().unwrap()).join("");
    println!("Part two: {}", part_two);
}
