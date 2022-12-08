use std::collections::HashSet;

fn scenic_score(tree_map: &[Vec<i32>], tx: usize, ty: usize) -> u32 {
    let width = tree_map[0].len();
    let height = tree_map.len();
    let treehouse_height = tree_map[ty][tx];
    // viewing scores are multiplied so if any are 0 its gonna be 0 in the end anyway
    if tx == 0 || tx == width - 1 || ty == 0 || ty == height - 1 {
        return 0;
    }
    let mut up_score: u32 = 0;
    let mut down_score: u32 = 0;
    let mut left_score: u32 = 0;
    let mut right_score: u32 = 0;
    // up
    for y in (0..ty).rev() {
        up_score += 1;
        if tree_map[y][tx] >= treehouse_height {
            break;
        }
    }
    // down
    for y in (ty + 1..height) {
        down_score += 1;
        if tree_map[y][tx] >= treehouse_height {
            break;
        }
    }
    // left
    for x in (0..tx).rev() {
        left_score += 1;
        if tree_map[ty][x] >= treehouse_height {
            break;
        }
    }
    // right
    for x in (tx + 1..width) {
        right_score += 1;
        if tree_map[ty][x] >= treehouse_height {
            break;
        }
    }
    up_score * down_score * left_score * right_score
}

fn part_2(tree_map: &[Vec<i32>]) {
    let width = tree_map[0].len();
    let height = tree_map.len();
    let mut highest_scenic_score: u32 = 0;
    (0..width).for_each(|x| {
        (0..height).for_each(|y| {
            let score = scenic_score(tree_map, x, y);
            if score > highest_scenic_score {
                println!("New highest score of {score} at x:{x} y:{y}");
                highest_scenic_score = score;
            }
        })
    });
    println!("Highest scenic score: {highest_scenic_score}");
}

fn main() {
    let lines = utils::read_arg_file_lines();
    let tree_map: Vec<Vec<i32>> = lines
        .iter()
        .map(|l| {
            l.chars()
                .map(|c| c.to_string().parse::<i32>().unwrap())
                .collect()
        })
        .collect();
    let width = tree_map[0].len();
    let height = tree_map.len();
    let mut visible_trees: HashSet<(usize, usize)> = HashSet::new();

    // From top
    (0..width).for_each(|x| {
        let mut highest: i32 = -1;
        (0..height).for_each(|y| {
            let tree_height = tree_map[x][y];
            if tree_height > highest {
                highest = tree_height;
                visible_trees.insert((x, y));
            }
        })
    });
    // From bottom
    (0..width).for_each(|x| {
        let mut highest: i32 = -1;
        (0..height).rev().for_each(|y| {
            let tree_height = tree_map[x][y];
            if tree_height > highest {
                highest = tree_height;
                visible_trees.insert((x, y));
            }
        })
    });
    // From left
    (0..height).for_each(|y| {
        let mut highest: i32 = -1;
        (0..width).for_each(|x| {
            let tree_height = tree_map[x][y];
            if tree_height > highest {
                highest = tree_height;
                visible_trees.insert((x, y));
            }
        })
    });
    // From right
    (0..height).for_each(|y| {
        let mut highest: i32 = -1;
        (0..width).rev().for_each(|x| {
            let tree_height = tree_map[x][y];
            if tree_height > highest {
                highest = tree_height;
                visible_trees.insert((x, y));
            }
        })
    });

    println!("Visible: {:?}", visible_trees);
    println!("Visible amount: {}", visible_trees.len());

    part_2(&tree_map);
}
