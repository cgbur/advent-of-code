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

    let mut counts = vec![];
    let mut sum = 0;

    for result in reader.lines() {
        let line = result?;

        if line.is_empty() {
            counts.push(sum);
            sum = 0;
        } else {
            sum += line.parse::<u32>().unwrap_or(0);
        }
    }
    // Don't forget the last group
    counts.push(sum);

    counts.sort_unstable_by(|a, b| b.cmp(a));

    println!("{}", counts[0]);

    let mut top_three = 0;
    for count in &counts[0..3] {
        top_three += count;
    }
    println!("{}", top_three);

    Ok(())
}
