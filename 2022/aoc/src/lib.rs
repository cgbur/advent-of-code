use once_cell;
use std::{fs::File, io::Read};

static INPUT: once_cell::sync::Lazy<String> = once_cell::sync::Lazy::new(|| {
    let calling_dir = std::env::current_dir().unwrap();
    let path = calling_dir.join("input");
    let mut file = File::open(path).unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    input
});

pub fn input() -> &'static str {
    &INPUT
}
