use itertools::Itertools;

pub fn run_part1(path: &str) -> i64 {
    let data = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|n| n.parse::<i32>().unwrap_or(-1))
        .collect_vec();

    solve_part1(data) as i64
}

fn solve_part1(inputs: Vec<i32>) -> i32 {
    let mut max = 0;
    let mut sum = 0;
    for num in inputs {
        if num == -1 {
            if sum > max {
                max = sum;
            }
            sum = 0;
        } else {
            sum += num;
        }
    }

    max
}

pub fn run_part2(path: &str) -> i64 {
    let data = parse_part2(std::fs::read_to_string(path).unwrap());

    solve_part2(data) as i64
}

fn parse_part2(input: String) -> Vec<Vec<i32>> {
    let vector = input
        .lines()
        .map(|n| n.parse::<i32>().unwrap_or(-1))
        .collect_vec();

    vector
        .split(|num| *num == -1)
        .map(|s| s.to_vec())
        .collect_vec()
}

fn solve_part2(inputs: Vec<Vec<i32>>) -> i32 {
    inputs
        .iter()
        .map(|v| v.iter().sum::<i32>())
        .sorted()
        .rev()
        .take(3)
        .sum()
}
