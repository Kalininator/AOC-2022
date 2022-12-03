fn item_value(item: char) -> u32 {
    let item_unicode = item as u32;
    println!("{item} unicode is {item_unicode}");
    if item_unicode > 96 {
        return item_unicode - 96;
    }
    item_unicode - 64 + 26
}

fn part_1(rucksacks: &Vec<String>) -> u32 {
    let mut total_priority: u32 = 0;
    for rucksack in rucksacks {
        let total_items = rucksack.len();
        let left = &rucksack[0..(total_items / 2)];
        let right = &rucksack[(total_items / 2)..total_items];
        println!("Rucksack content: {rucksack}");
        println!("Left items: {left:?}");
        println!("Right items: {right:?}");
        for item in left.chars() {
            if right.contains(item) {
                let item_value = item_value(item);
                total_priority += item_value;
                println!("Item {item} with value {item_value} is in both compartments");
                break;
            }
        }
    }
    total_priority
}

fn find_badge_type(rucksacks: &[String]) -> char {
    for item in rucksacks[0].chars() {
        if rucksacks[1].contains(item) && rucksacks[2].contains(item) {
            return item;
        }
    }
    panic!("These 3 rucksacks do not have a badge type");
}

fn part_2(rucksacks: &[String]) -> u32 {
    // find badge type
    let groups = rucksacks.chunks(3);
    let mut badge_priorities: u32 = 0;
    for group in groups {
        let badge_type = find_badge_type(group);
        badge_priorities += item_value(badge_type);
        println!("Badge type: {badge_type}");
    }
    badge_priorities
}

fn main() {
    let rucksacks = utils::read_arg_file_lines();
    let p1 = part_1(&rucksacks);
    let p2 = part_2(&rucksacks);
    println!("Part 1 result: {p1}");
    println!("Part 2 result: {p2}");
}
