#[macro_use]
extern crate scan_fmt;

fn main() {
    let unparsed_elf_pairs = utils::read_arg_file_lines();
    let mut complete_overlaps: usize = 0;
    let mut partial_overlaps: usize = 0;
    for line in unparsed_elf_pairs {
        if let Ok((a, b, c, d)) = scan_fmt!(&line, "{d}-{d},{d}-{d}", usize, usize, usize, usize) {
            if (a <= c && b >= d) || (c <= a && d >= b) {
                complete_overlaps += 1;
                partial_overlaps += 1;
                continue;
            }
            if (b >= c && a <= c) || (a <= d && b >= d) {
                partial_overlaps += 1;
            }
        }
    }
    println!("Completely overlapping elf pairs: {complete_overlaps}");
    println!("Partially overlapping elf pairs: {partial_overlaps}");
}
