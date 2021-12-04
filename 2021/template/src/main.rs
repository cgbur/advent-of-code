use itertools::Itertools;
use std::error::Error;
use std::fs::File;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
    let inputs = parse_input()?;

    Ok(())
}

fn parse_input() -> Result<(), std::io::Error> {
    let mut input = String::new();
    File::open("./input")?.read_to_string(&mut input)?;

    Ok(())
}
