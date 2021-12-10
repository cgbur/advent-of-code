use itertools::Itertools;
use std::error::Error;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Clone)]
enum ParseResult {
    Valid,
    Invalid(char),
    Incomplete(Vec<char>),
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    File::open("./input")?.read_to_string(&mut input)?;

    let parsed = input.lines().map(parse_line).collect_vec();

    let part_one = parsed
        .iter()
        .filter_map(|res| match res {
            ParseResult::Invalid(token) => match token {
                '>' => Some(25137),
                ')' => Some(3),
                ']' => Some(57),
                '}' => Some(1197),
                _ => panic!("unknown token"),
            },
            _ => None,
        })
        .sum::<u32>();

    let scores = parsed
        .iter()
        .filter_map(|res| {
            if let ParseResult::Incomplete(rest) = res {
                Some(rest)
            } else {
                None
            }
        })
        .map(|tokens| {
            let mut score = 0u64;

            for token in tokens.iter().rev() {
                score *= 5;
                score += match token {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => panic!("unknown token"),
                }
            }

            score
        })
        .sorted()
        .collect_vec();

    let part_two = scores[scores.len() / 2];

    println!("{}", part_one);
    println!("{}", part_two);

    Ok(())
}

fn parse_line(line: &str) -> ParseResult {
    let mut stack = vec![];

    for token in line.chars() {
        match token {
            '}' => {
                if *stack.last().unwrap() != '{' {
                    return ParseResult::Invalid('}');
                } else {
                    stack.pop();
                }
            }
            ')' => {
                if *stack.last().unwrap() != '(' {
                    return ParseResult::Invalid(')');
                } else {
                    stack.pop();
                }
            }
            ']' => {
                if *stack.last().unwrap() != '[' {
                    return ParseResult::Invalid(']');
                } else {
                    stack.pop();
                }
            }
            '>' => {
                if *stack.last().unwrap() != '<' {
                    return ParseResult::Invalid('>');
                } else {
                    stack.pop();
                }
            }
            _ => stack.push(token),
        }
    }

    if stack.is_empty() {
        ParseResult::Valid
    } else {
        ParseResult::Incomplete(stack)
    }
}
