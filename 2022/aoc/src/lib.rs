use once_cell;
use std::{fs::File, io::Read};

static INPUT: once_cell::sync::Lazy<String> = once_cell::sync::Lazy::new(|| {
    // if cargo manifest dir does not end in 2022, pop the last path component
    let mut root = std::path::Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap()).to_owned();
    if !root.ends_with("2022") {
        root.pop();
    }
    let binary_name = std::env::current_exe().unwrap();
    let sub_dir = binary_name
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .strip_prefix("day")
        .unwrap();
    let path = root.join(sub_dir).join("input");
    let mut file = File::open(path).unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    input
});

pub fn input() -> &'static str {
    &INPUT
}
