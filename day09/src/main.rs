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

#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy, Default)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Node {
    position: Position,
    history: HashSet<Position>,
}
impl Default for Node {
    fn default() -> Self {
        let mut history = HashSet::new();
        history.insert(Position::default());
        Self {
            position: Position::default(),
            history,
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
}

fn distance(head: &Node, tail: &Node) -> u32 {
    let h_dist = (head.position.x - tail.position.x).abs();
    let v_dist = (head.position.y - tail.position.y).abs();
    std::cmp::max(h_dist, v_dist).try_into().unwrap()
}

fn move_tail(tail: &mut Node, head: &Node, direction: &Direction) {
    match direction {
        Direction::Up => {
            tail.position.x = head.position.x;
            tail.position.y = head.position.y - 1;
        }
        Direction::Down => {
            tail.position.x = head.position.x;
            tail.position.y = head.position.y + 1;
        }
        Direction::Left => {
            tail.position.y = head.position.y;
            tail.position.x = head.position.x + 1;
        }
        Direction::Right => {
            tail.position.y = head.position.y;
            tail.position.x = head.position.x - 1;
        }
    };
    tail.record_tail();
}

fn main() {
    let lines = utils::read_arg_file_lines();
    let instructions: Vec<Instruction> = lines
        .iter()
        .map(|l| sscanf!(&l, "{Instruction}").unwrap())
        .collect();

    let mut head: Node = Node::default();
    let mut tail: Node = Node::default();

    for i in instructions {
        for _ in 0..i.distance {
            println!("{:?}", i.direction);
            move_head(&mut head, &i.direction);
            println!("Head at: {:?}", head.position);
            if distance(&head, &tail) > 1 {
                println!("Tail needs to move");
                move_tail(&mut tail, &head, &i.direction);
                println!("tail moved to {:?}", tail.position);
            }
        }
    }
    println!("Tail positions: {}", tail.history.len());
}
