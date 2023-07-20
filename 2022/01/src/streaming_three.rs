use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    // check args length to make sure it has a file name
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <input file>", args[0]);
        std::process::exit(1);
    }

    let file = File::open(&args[1])?;
    let reader = BufReader::new(file);

    let mut counts = [0u32; 3];
    let mut sum = 0;

    for result in reader.lines() {
        let line = result?;

        if line.is_empty() {
            replace_smallest(&mut counts, sum);
            sum = 0;
        } else {
            sum += line.parse::<u32>().unwrap_or(0);
        }
    }
    // Don't forget the last group
    replace_smallest(&mut counts, sum);

    counts.sort_unstable_by(|a, b| b.cmp(a));
    println!("{}", counts[0]);
    println!("{}", counts.iter().sum::<u32>());

    Ok(())
}

fn replace_smallest(counts: &mut [u32; 3], sum: u32) {
    let mut smallest = 0;
    for (index, count) in counts.iter().enumerate() {
        if count < &counts[smallest] {
            smallest = index;
        }
    }
    if sum > counts[smallest] {
        counts[smallest] = sum;
    }
}
