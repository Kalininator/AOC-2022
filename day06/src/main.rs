fn marker_location(input: &str, marker_size: usize) -> Option<usize> {
    let chars: Vec<char> = input.chars().collect();
    for (i, w) in chars.windows(marker_size).enumerate() {
        let mut section = w.to_vec();
        section.sort_unstable();
        section.dedup();
        if section.len() == marker_size {
            return Some(i + marker_size);
        }
    }
    None
}
fn main() {
    let line = &utils::read_arg_file_lines()[0];
    println!("Part 1: {:?}", marker_location(line, 4));
    println!("Part 2: {:?}", marker_location(line, 14));
}
