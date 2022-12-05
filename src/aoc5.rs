use std::collections::VecDeque;

use itertools::Itertools;

pub fn run_part1(path: &str) -> i64 {
    let input = parse_data(std::fs::read_to_string(path).unwrap());
    let ret = solve_part1(input);

    println!("Result part1: {}", ret);

    0
}

pub fn run_part2(path: &str) -> i64 {
    let input = parse_data(std::fs::read_to_string(path).unwrap());
    let ret = solve_part2(input);

    println!("Result part2: {}", ret);

    0
}

struct Cargo {
    stacks: Vec<VecDeque<char>>,
    moves: Vec<[u32; 3]>,
}

fn parse_data(data: String) -> Cargo {
    let parts: Vec<&str> = data.split("\n\n").collect();
    let columns = parts[0]
        .split(' ')
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();

    // Parse stacks
    let mut stacks: Vec<VecDeque<char>> = vec![VecDeque::new(); columns];
    for row in parts[0].lines().take(parts[0].lines().count() - 1) {
        let elements = row.chars().collect_vec();
        for i in 0..columns {
            if let Some(c) = elements.get((i * 4) + 1) {
                if c.is_alphabetic() {
                    stacks[i].push_back(*c);
                }
            }
        }
    }

    // Parse moves
    let moves = parts[1]
        .lines()
        .map(|l| l.split(' '))
        .map(|mut s| {
            [
                s.nth(1).unwrap().parse::<u32>().unwrap(),
                s.nth(1).unwrap().parse::<u32>().unwrap(),
                s.nth(1).unwrap().parse::<u32>().unwrap(),
            ]
        })
        .collect_vec();

    Cargo { stacks, moves }
}

fn solve_part1(mut input: Cargo) -> String {
    for op in input.moves {
        let src = (op[1] - 1) as usize;
        let dst = (op[2] - 1) as usize;

        for _ in 0..op[0] {
            let top = input.stacks[src].pop_front().unwrap();
            input.stacks[dst].push_front(top);
        }
    }

    input.stacks.iter().map(|s| s.front().unwrap()).collect()
}
fn solve_part2(mut input: Cargo) -> String {
    for op in input.moves {
        let src = (op[1] - 1) as usize;
        let dst = (op[2] - 1) as usize;

        let mut temp = VecDeque::new();
        for _ in 0..op[0] {
            temp.push_front(input.stacks[src].pop_front().unwrap());
        }
        for c in temp {
            input.stacks[dst].push_front(c);
        }
    }

    input.stacks.iter().map(|s| s.front().unwrap()).collect()
}
