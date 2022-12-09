use sscanf::sscanf;
use std::collections::HashSet;

#[derive(Debug, sscanf::FromScanf, Clone, Copy)]
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

#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy, Default)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Node {
    position: Position,
    history: HashSet<Position>,
    last_direction_moved: Option<Direction>,
}
impl Default for Node {
    fn default() -> Self {
        let mut history = HashSet::new();
        history.insert(Position::default());
        Self {
            position: Position::default(),
            history,
            last_direction_moved: None,
        }
    }
}
impl Node {
    fn record_tail(&mut self) {
        self.history.insert(self.position);
    }
}

fn move_head(head: &mut Node, direction: &Direction) {
    match direction {
        Direction::Up => head.position.y += 1,
        Direction::Down => head.position.y -= 1,
        Direction::Left => head.position.x -= 1,
        Direction::Right => head.position.x += 1,
    };
    head.last_direction_moved = Some(*direction);
}

fn distance(head: &Position, tail: &Node) -> u32 {
    let h_dist = (head.x - tail.position.x).abs();
    let v_dist = (head.y - tail.position.y).abs();
    std::cmp::max(h_dist, v_dist).try_into().unwrap()
}

fn clamp(input: i32, min: i32, max: i32) -> i32 {
    if input >= max {
        return max;
    }
    if input <= min {
        return min;
    }
    input
}

fn move_tail(tail: &mut Node, head: &Position) {
    let distance: Position = Position {
        x: head.x - tail.position.x,
        y: head.y - tail.position.y,
    };
    tail.position.x += clamp(distance.x, -1, 1);
    tail.position.y += clamp(distance.y, -1, 1);
    tail.record_tail();
}

fn part_1(instructions: &Vec<Instruction>) {
    let mut head: Node = Node::default();
    let mut tail: Node = Node::default();

    for i in instructions {
        for _ in 0..i.distance {
            // println!("{:?}", i.direction);
            move_head(&mut head, &i.direction);
            // println!("Head at: {:?}", head.position);
            if distance(&head.position, &tail) > 1 {
                // println!("Tail needs to move");
                move_tail(&mut tail, &head.position);
                // println!("tail moved to {:?}", tail.position);
            }
        }
    }
    println!("Part 1 tail positions: {}", tail.history.len());
}

fn part_2(instructions: &Vec<Instruction>, tails: usize) {
    let mut nodes: Vec<Node> = Vec::new();
    for _ in 0..tails + 1 {
        nodes.push(Node::default());
    }

    for i in instructions {
        for _ in 0..i.distance {
            // println!("{:?}", i.direction);
            move_head(&mut nodes[0], &i.direction);
            for node_index in 1..nodes.len() {
                let head_position = nodes[node_index - 1].position;
                let dist = distance(&head_position, &nodes[node_index]);
                // if node_index == tails {
                //     println!("distance: {dist}");
                // }
                if dist > 1 {
                    move_tail(&mut nodes[node_index], &head_position);
                }
            }
        }
    }
    println!(
        "Part 2 tail positions: {}",
        nodes.last().unwrap().history.len()
    );
}

fn main() {
    let lines = utils::read_arg_file_lines();
    let instructions: Vec<Instruction> = lines
        .iter()
        .map(|l| sscanf!(&l, "{Instruction}").unwrap())
        .collect();
    part_1(&instructions);
    part_2(&instructions, 1);
    part_2(&instructions, 9);
}
