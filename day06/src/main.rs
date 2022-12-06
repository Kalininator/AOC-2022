fn main() {
    let line = &utils::read_arg_file_lines()[0];
    println!("{line}");
    let chars: Vec<char> = line.chars().collect();
    println!("{chars:?}");
    for (i, w) in chars.windows(4).enumerate() {
        let mut chunk = w.to_vec();
        chunk.sort();
        chunk.dedup();
        if chunk.len() == 4 {
            println!("Part 1: {}", i + 4);
            break;
        }
    }

    for (i, w) in chars.windows(14).enumerate() {
        let mut chunk = w.to_vec();
        chunk.sort();
        chunk.dedup();
        if chunk.len() == 14 {
            println!("Part 2: {}", i + 14);
            break;
        }
    }
}
