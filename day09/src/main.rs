use sscanf::sscanf;
use std::collections::HashSet;

#[derive(Debug, sscanf::FromScanf)]
enum Direction {
    #[sscanf(format = "U")]
    Up,
    #[sscanf(format = "D")]
    Down,
    #[sscanf(format = "L")]
    Left,
    #[sscanf(format = "R")]
    Right,
}

#[derive(Debug, sscanf::FromScanf)]
#[sscanf(format = "{direction} {distance}")]
struct Instruction {
    direction: Direction,
    distance: u32,
}

fn move_head(head: &mut (i32, i32), direction: &Direction) {
    match direction {
        Direction::Up => head.1 += 1,
        Direction::Down => head.1 -= 1,
        Direction::Left => head.0 -= 1,
        Direction::Right => head.0 += 1,
    };
}

fn distance(head: &(i32, i32), tail: &(i32, i32)) -> u32 {
    let h_dist = (head.0 - tail.0).abs();
    let v_dist = (head.1 - tail.1).abs();
    std::cmp::max(h_dist, v_dist).try_into().unwrap()
}

fn move_tail(tail: &mut (i32, i32), head: &(i32, i32), direction: &Direction) {
    match direction {
        Direction::Up => {
            tail.0 = head.0;
            tail.1 = head.1 - 1;
        }
        Direction::Down => {
            tail.0 = head.0;
            tail.1 = head.1 + 1;
        }
        Direction::Left => {
            tail.1 = head.1;
            tail.0 = head.0 + 1;
        }
        Direction::Right => {
            tail.1 = head.1;
            tail.0 = head.0 - 1;
        }
    };
}

fn main() {
    let lines = utils::read_arg_file_lines();
    let instructions: Vec<Instruction> = lines
        .iter()
        .map(|l| sscanf!(&l, "{Instruction}").unwrap())
        .collect();

    let mut tail_positions: HashSet<(i32, i32)> = HashSet::new();
    tail_positions.insert((0, 0));

    let mut head: (i32, i32) = (0, 0);
    let mut tail: (i32, i32) = (0, 0);

    for i in instructions {
        for _ in 0..i.distance {
            println!("{:?}", i.direction);
            move_head(&mut head, &i.direction);
            println!("Head at: {head:?}");
            if distance(&head, &tail) > 1 {
                println!("Tail needs to move");
                move_tail(&mut tail, &head, &i.direction);
                tail_positions.insert(tail);
                println!("tail moved to {tail:?}");
            }
        }
    }
    println!("Tail positions: {}", tail_positions.len());
}
