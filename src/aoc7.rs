use itertools::Itertools;

pub fn run_part1(path: &str) -> i64 {
    let input = parse_data(std::fs::read_to_string(path).unwrap());

    solve_part1(input) as i64
}

pub fn run_part2(path: &str) -> i64 {
    let input = parse_data(std::fs::read_to_string(path).unwrap());

    solve_part2(input) as i64
}

#[derive(Debug)]
enum Command {
    Cd(String),
    Ls,
    Invalid,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Node {
    File(usize, String),
    Dir(Vec<usize>, String),
}

type Filesystem = Vec<Node>;

fn parse_command(line: &str) -> Command {
    let command = line.split_whitespace().collect_vec();

    match command[1] {
        "cd" => Command::Cd(command[2].to_string()),
        "ls" => Command::Ls,
        _ => Command::Invalid,
    }
}

fn new_node(line: &str) -> Node {
    let line = line.split_whitespace().collect_vec();

    if line[0] == "dir" {
        Node::Dir(Vec::new(), line[1].to_string())
    } else {
        Node::File(line[0].parse::<usize>().unwrap(), line[1].to_string())
    }
}

fn parse_data(data: String) -> Filesystem {
    let mut fs = vec![Node::Dir(Vec::new(), "/".to_string())];
    let mut global_path: Vec<usize> = vec![0];

    for line in data.lines().skip(1) {
        // Command
        if line.starts_with('$') {
            if let Command::Cd(dir) = parse_command(line) {
                if dir == ".." {
                    global_path.pop();
                } else {
                    let parent = &fs[*global_path.last().unwrap()];

                    if let Node::Dir(inodes, _) = parent {
                        let index = inodes
                            .iter()
                            .find(|x| {
                                if let Node::Dir(_, ref b) = fs[**x] {
                                    b == &dir
                                } else {
                                    false
                                }
                            })
                            .unwrap();
                        global_path.push(*index);
                    }
                }
            }
        }
        // List file
        else {
            let node = new_node(line);
            let dir_index = *global_path.last().unwrap();
            let fs_len = fs.len();
            if let Node::Dir(ref mut inodes, _) = fs[dir_index] {
                inodes.push(fs_len);
            }
            fs.push(node);
        }
    }

    fs
}

fn tree_traverse(fs: &Filesystem, root: &Node, acc: &mut Vec<usize>) -> usize {
    let mut sum = 0;

    if let Node::Dir(inodes, _) = root {
        for inode in inodes {
            match fs[*inode] {
                Node::File(size, _) => sum += size,
                Node::Dir(_, _) => sum += tree_traverse(fs, &fs[*inode], acc),
            }
        }
    }

    acc.push(sum);
    sum
}

fn calculate_dir_sizes(fs: &Filesystem) -> Vec<usize> {
    let mut sizes = Vec::new();

    tree_traverse(fs, &fs[0], &mut sizes);

    sizes
}

fn solve_part1(input: Filesystem) -> usize {
    let sizes = calculate_dir_sizes(&input);

    sizes.iter().filter(|s| **s <= 100000).sum()
}

fn solve_part2(input: Filesystem) -> usize {
    const TOTAL_SPACE: usize = 70000000;
    const NEEDED_SPACE: usize = 30000000;

    let mut sizes = calculate_dir_sizes(&input);
    sizes.sort();

    let occupied = sizes.last().unwrap();

    let needed = occupied - (TOTAL_SPACE - NEEDED_SPACE);

    for size in sizes {
        if size >= needed {
            return size;
        }
    }

    unreachable!()
}
