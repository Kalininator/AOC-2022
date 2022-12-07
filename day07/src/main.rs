use sscanf::sscanf;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, sscanf::FromScanf)]
enum Destination {
    #[sscanf(format = "/")]
    Root,

    #[sscanf(format = "..")]
    Parent,

    #[sscanf(format = "{}")]
    Child(String),
}

#[derive(Debug, sscanf::FromScanf)]
enum Line {
    #[sscanf(format = "$ cd {}")]
    Cd(Destination),

    #[sscanf(format = "$ ls")]
    Ls,

    #[sscanf(format = "{size} {name}")]
    File { size: usize, name: String },

    #[sscanf(format = "dir {}")]
    Directory(String),
}

fn folder_path(dir: &[String]) -> String {
    if dir.is_empty() {
        return "".to_string();
    }
    format!("/{}", dir.join("/"))
}
fn path(dir: &[String], filename: String) -> String {
    if dir.is_empty() {
        return format!("/{filename}");
    }
    format!("/{}/{filename}", dir.join("/"))
}

fn folder_size(folder: &str, files: &HashMap<String, usize>) -> usize {
    files
        .iter()
        .filter(|&(path, _size)| path.starts_with(&folder))
        .map(|(_path, size)| size)
        .sum()
}

fn main() {
    let lines: Vec<Line> = utils::read_arg_file_lines()
        .iter()
        .map(|l| sscanf!(l, "{Line}").expect("Invalid terminal line"))
        .collect();
    println!("Lines:");
    for line in &lines {
        println!("{line:?}");
    }

    let mut current_dir: Vec<String> = vec![];
    let mut files: HashMap<String, usize> = HashMap::new();
    let mut folders: HashSet<String> = HashSet::new();
    folders.insert("/".to_string());
    for line in lines {
        match line {
            Line::Cd(to) => {
                match to {
                    Destination::Parent => {
                        current_dir.pop();
                    }
                    Destination::Root => {
                        current_dir.drain(0..);
                    }
                    Destination::Child(dir) => {
                        current_dir.push(dir.to_string());
                    }
                };
            }
            Line::File { size, name } => {
                let file_path = path(&current_dir, name);
                files.insert(file_path, size);
            }
            Line::Directory(name) => {
                folders.insert(format!("{}/{name}", folder_path(&current_dir)));
            }
            _ => (),
        }
    }

    let part1: usize = folders
        .iter()
        .map(|f| folder_size(f, &files))
        .filter(|size| size <= &100000)
        .sum();
    println!("Part 1: {part1:?}");

    let total_used_space = folder_size(&String::from("/"), &files);
    let total_space = 70000000;
    let free_space = total_space - total_used_space;
    let space_to_free = 30000000 - free_space;
    let smallest_big_enough_folder: usize = folders
        .iter()
        .map(|f| folder_size(f, &files))
        .filter(|size| size >= &space_to_free)
        .min()
        .unwrap();
    println!("Part 2: {smallest_big_enough_folder}");
}
