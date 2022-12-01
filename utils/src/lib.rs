use std::fs;
use std::io::{BufRead, BufReader};

pub fn read_arg_file_lines() -> Vec<String> {
    let args: Vec<String> = std::env::args().collect();
    let file = fs::File::open(&args[1]).expect("Failed to read file");
    let br = BufReader::new(file);

    br.lines().map(|l| l.expect("Failed to get line")).collect()
}
