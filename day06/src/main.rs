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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example_1() {
        assert_eq!(marker_location("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), Some(5));
    }
    #[test]
    fn part_1_example_2() {
        assert_eq!(marker_location("nppdvjthqldpwncqszvftbrmjlhg", 4), Some(6));
    }
    #[test]
    fn part_1_example_3() {
        assert_eq!(
            marker_location("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4),
            Some(10)
        );
    }
    #[test]
    fn part_1_example_4() {
        assert_eq!(
            marker_location("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4),
            Some(11)
        );
    }

    #[test]
    fn part_2_example_1() {
        assert_eq!(
            marker_location("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14),
            Some(19)
        );
    }
    #[test]
    fn part_2_example_2() {
        assert_eq!(
            marker_location("bvwbjplbgvbhsrlpgdmjqwftvncz", 14),
            Some(23)
        );
    }
    #[test]
    fn part_2_example_3() {
        assert_eq!(
            marker_location("nppdvjthqldpwncqszvftbrmjlhg", 14),
            Some(23)
        );
    }
    #[test]
    fn part_2_example_4() {
        assert_eq!(
            marker_location("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14),
            Some(29)
        );
    }
    #[test]
    fn part_2_example_5() {
        assert_eq!(
            marker_location("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14),
            Some(26)
        );
    }
}
