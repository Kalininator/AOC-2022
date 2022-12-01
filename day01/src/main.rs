fn main() {
    let args: Vec<String> = std::env::args().collect();
    let lines = utils::read_file(&args[1]);

    let split_sections: Vec<Vec<usize>> = lines
        .split(|l| l.is_empty())
        .map(|section| {
            section
                .iter()
                .map(|v| v.parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    let mut summed_sections: Vec<usize> = split_sections
        .iter()
        .map(|section| section.iter().sum())
        .collect::<Vec<usize>>();
    summed_sections.sort();
    summed_sections.reverse();

    let most_calories = summed_sections[0];
    println!("Elf carrying most calories is carrying {most_calories} calories.");

    let top_3_elf_total_calories: usize = summed_sections.iter().take(3).sum();
    println!("Top 3 elves are carrying a total of {top_3_elf_total_calories} calories.");
}
