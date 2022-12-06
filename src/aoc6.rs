pub fn run_part1(path: &str) -> i64 {
    let input = std::fs::read_to_string(path).unwrap();
    solve_part1(input) as i64
}

pub fn run_part2(path: &str) -> i64 {
    let input = std::fs::read_to_string(path).unwrap();

    solve_part2(input) as i64
}

fn check_unique(slice: &[u8]) -> bool {
    for i in 0..slice.len() {
        if slice[i + 1..].contains(&slice[i]) {
            return false;
        }
    }
    true
}

fn solve_part1(input: String) -> i32 {
    let input = input.as_bytes();
    for i in 0..(input.len() - 4) {
        if check_unique(&input[i..i + 4]) {
            return i as i32 + 4;
        }
    }
    0
}

fn solve_part2(input: String) -> i32 {
    let input = input.as_bytes();
    for i in 0..(input.len() - 14) {
        if check_unique(&input[i..i + 14]) {
            return i as i32 + 14;
        }
    }
    0
}
