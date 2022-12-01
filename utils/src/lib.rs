use std::fs;
use std::io::{BufRead, BufReader};

pub fn read_arg_file_lines() -> Vec<String> {
    let args: Vec<String> = std::env::args().collect();
    let file = fs::File::open(&args[1]).expect("Failed to read file");
    let br = BufReader::new(file);

    br.lines().map(|l| l.expect("Failed to get line")).collect()
}

pub fn read_full_file(path: &str) -> String {
    fs::read_to_string(path).expect("Failed to read file")
}

pub fn binary_to_decimal(bools: Vec<bool>) -> u32 {
    let mut acc: u32 = 0;
    for i in bools {
        match i {
            true => acc = (acc * 2) + 1,
            false => acc *= 2,
        }
    }
    acc
}
