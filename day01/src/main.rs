fn main() {
    let lines = utils::read_arg_file_lines();

    let calories_by_elf: Vec<Vec<usize>> = lines
        .split(|l| l.is_empty())
        .map(|section| section.iter().map(|v| v.parse().unwrap()).collect())
        .collect();

    let mut elf_total_calories: Vec<usize> = calories_by_elf
        .iter()
        .map(|section| section.iter().sum())
        .collect();
    elf_total_calories.sort_unstable();
    elf_total_calories.reverse();

    let most_calories = elf_total_calories[0];
    println!("Elf carrying most calories is carrying {most_calories} calories.");

    let top_3_elf_total_calories: usize = elf_total_calories.iter().take(3).sum();
    println!("Top 3 elves are carrying a total of {top_3_elf_total_calories} calories.");
}
