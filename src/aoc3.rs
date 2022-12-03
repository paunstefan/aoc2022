use itertools::Itertools;

pub fn run_part1(path: &str) -> i64 {
    let file = std::fs::read_to_string(path).unwrap();
    let data = file.lines().collect_vec();

    solve_part1(data) as i64
}

pub fn run_part2(path: &str) -> i64 {
    let file = std::fs::read_to_string(path).unwrap();
    let data = file.lines().collect_vec();
    solve_part2(data) as i64
}

fn char_priority(c: char) -> i32 {
    if c.is_uppercase() {
        ((c as u8) - b'A' + 27) as i32
    } else {
        ((c as u8) - b'a' + 1) as i32
    }
}

fn solve_part1(data: Vec<&str>) -> i32 {
    data.iter()
        .map(|l| l.split_at(l.len() / 2))
        .map(|s| s.0.chars().find(|&c| s.1.contains(c)))
        .map(|c| char_priority(c.unwrap()))
        .sum()
}

fn solve_part2(data: Vec<&str>) -> i32 {
    data.chunks(3)
        .map(|g| g[0].chars().find(|&c| g[1].contains(c) && g[2].contains(c)))
        .map(|c| char_priority(c.unwrap()))
        .sum()
}
