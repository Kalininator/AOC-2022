use sscanf::sscanf;

#[derive(sscanf::FromScanf, Debug)]
#[sscanf(format = "move {amount} from {from} to {to}")]
struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

fn main() {
    let all_lines = utils::read_arg_file_lines();
    let initial_line_split: Vec<Vec<String>> = all_lines
        .split(|line| line.is_empty())
        .map(|l| l.to_vec())
        .collect();
    let column_index_line = initial_line_split[0].to_vec().pop().unwrap();
    let column_count = column_index_line.chars().filter(|c| c != &' ').count();
    let mut chars: Vec<Vec<char>> = vec![vec![]; column_count];
    for line in &initial_line_split[0][0..initial_line_split[0].len() - 1] {
        let cs: Vec<char> = line.chars().collect();
        for (i, section) in cs.chunks(4).enumerate() {
            if section[1] != ' ' {
                chars[i].push(section[1]);
            }
        }
    }
    for i in 0..chars.len() {
        chars[i].reverse();
    }
    println!("Columns: {column_count}");
    println!("Columns: {chars:?}");
    println!("Instructions: ");
    let moves: Vec<Move> = initial_line_split[1]
        .iter()
        .map(|line| sscanf!(line, "{Move}").unwrap())
        .collect();
    println!("{:?}", moves);

    for m in moves {
        for _ in 0..m.amount {
            let c = chars[m.from - 1].pop().unwrap();
            chars[m.to - 1].push(c);
        }
    }
    println!("Result: ");
    for c in chars {
        print!("{}", c.last().unwrap());
    }
    // let foo = sscanf!("move 1 from 2 to 1", "{Move}").unwrap();
    // println!("{} {} {}", foo.amount, foo.from, foo.to);
}
