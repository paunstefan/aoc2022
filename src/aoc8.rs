use itertools::Itertools;

pub fn run_part1(path: &str) -> i64 {
    let input = parse_data(std::fs::read_to_string(path).unwrap());

    solve_part1(input) as i64
}

pub fn run_part2(path: &str) -> i64 {
    let input = parse_data(std::fs::read_to_string(path).unwrap());

    solve_part2(input) as i64
}

type Forest = Vec<Vec<u8>>;

fn parse_data(data: String) -> Forest {
    data.lines()
        .map(|l| l.bytes().map(|c| c - b'0').collect_vec())
        .collect_vec()
}

fn solve_part1(input: Forest) -> i32 {
    let mut visible = (input.len() * 2) + (input[0].len() - 2) * 2;

    for row in 1..(input.len() - 1) {
        for col in 1..(input[0].len() - 1) {
            let mut left = true;
            // Check row
            for i in 0..col {
                if input[row][i] >= input[row][col] {
                    left = false;
                    break;
                }
            }
            let mut right = true;
            for i in (col + 1)..input[0].len() {
                if input[row][i] >= input[row][col] {
                    right = false;
                    break;
                }
            }

            // Check col
            let mut top = true;
            for j in 0..row {
                if input[j][col] >= input[row][col] {
                    top = false;
                    break;
                }
            }
            let mut down = true;
            for j in (row + 1)..input.len() {
                if input[j][col] >= input[row][col] {
                    down = false;
                    break;
                }
            }

            visible += (left || right || top || down) as usize;
        }
    }

    visible as i32
}
fn solve_part2(input: Forest) -> i32 {
    let mut result = 0;
    for row in 0..input.len() {
        for col in 0..input[0].len() {
            let mut left = 0;
            // Check row
            for i in (0..col).rev() {
                if input[row][i] < input[row][col] {
                    left += 1;
                } else {
                    left += 1;
                    break;
                }
            }
            let mut right = 0;
            for i in (col + 1)..input[0].len() {
                if input[row][i] < input[row][col] {
                    right += 1;
                } else {
                    right += 1;
                    break;
                }
            }

            // Check col
            let mut top = 0;
            for j in (0..row).rev() {
                if input[j][col] < input[row][col] {
                    top += 1;
                } else {
                    top += 1;
                    break;
                }
            }
            let mut down = 0;
            for j in (row + 1)..input.len() {
                if input[j][col] < input[row][col] {
                    down += 1;
                } else {
                    down += 1;
                    break;
                }
            }

            let visible = left * right * top * down;
            if visible > result {
                result = visible;
            }
        }
    }
    result
}
