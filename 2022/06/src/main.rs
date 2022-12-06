use aoc::input;
use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    println!("part one: {}", fancy(input(), 4));
    println!("part one: {}", fast(input(), 4));
    println!("part two: {}", fancy(input(), 14));
    println!("part two: {}", fast(input(), 14));
}

fn fancy(input: &str, size: usize) -> usize {
    let mut chars = vec![None; size];
    for (idx, char) in input.chars().enumerate() {
        chars[idx % size] = Some(char);
        if chars
            .iter()
            .filter_map(|c| *c)
            .sorted_unstable()
            .dedup()
            .count()
            == chars.len()
        {
            return idx + 1;
        }
    }
    panic!("no solution found");
}

/// In some benchmarks, around 7x faster than fancy. Leaving as an exercise to
/// the reader to figure out how to implement this using the ring buffer from
/// the fancy solution to avoid the allocation.
fn fast(input: &str, size: usize) -> usize {
    let chars = input.chars().collect_vec();
    let mut seen_chars: HashMap<char, usize> = HashMap::default();
    let mut start = 0;
    let mut end = 0;
    while end < chars.len() {
        let char = chars[end];
        if let Some(last_position) = seen_chars.insert(char, end) {
            for i in start..last_position {
                seen_chars.remove(&chars[i]);
            }
            start = last_position + 1;
        }
        if end - start + 1 == size {
            return end + 1;
        }
        end += 1;
    }
    panic!("no solution found");
}
