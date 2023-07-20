const INPUT: &str = include_str!("input");

fn main() -> std::io::Result<()> {
    let mut counts = vec![];

    for group in INPUT.split("\n\n") {
        let mut sum = 0;
        for line in group.split("\n") {
            sum += line.parse::<u32>().unwrap_or(0);
        }
        counts.push(sum);
    }

    counts.sort_unstable_by(|a, b| b.cmp(a));

    println!("{}", counts[0]);

    let mut top_three = 0;
    for count in &counts[0..3] {
        top_three += count;
    }
    println!("{}", top_three);

    Ok(())
}
