use sscanf::sscanf;

#[derive(Debug, sscanf::FromScanf)]
enum Instruction {
    #[sscanf(format = "addx {}")]
    Addx(i32),
    #[sscanf(format = "noop")]
    Noop,
}

fn main() {
    let mut instructions: Vec<Instruction> = utils::read_arg_file_lines()
        .iter()
        .map(|l| sscanf!(&l, "{Instruction}").unwrap())
        .rev()
        .collect();

    let watched_cycles: Vec<i32> = vec![20, 60, 100, 140, 180, 220];
    let mut total_signal: i32 = 0;

    let mut cycle: i32 = 1;
    let mut register: i32 = 1;
    let mut processing: bool = false;

    let mut pixels: Vec<char> = vec![];

    loop {
        if watched_cycles.contains(&cycle) {
            total_signal += register * cycle;
        }
        if instructions.is_empty() {
            break;
        }
        let i = instructions.last().unwrap();

        let pos = (cycle - 1) % 40;
        if pos <= register + 1 && pos >= register - 1 {
            pixels.push('#');
        } else {
            pixels.push(' ');
        }

        match i {
            Instruction::Addx(v) => {
                if processing {
                    register += v;
                    instructions.pop();
                    processing = false;
                } else {
                    processing = true;
                }
            }
            Instruction::Noop => {
                instructions.pop();
            }
        };

        cycle += 1;
    }

    println!("{total_signal}");
    for (i, p) in pixels.iter().enumerate() {
        if i % 40 == 0 {
            println!();
        }
        print!("{p}");
    }
}
