use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

pub fn run_part1(path: &str) -> i64 {
    let input = parse_data(std::fs::read_to_string(path).unwrap());

    solve_part1(input) as i64
}

pub fn run_part2(path: &str) -> i64 {
    let input = parse_data(std::fs::read_to_string(path).unwrap());

    solve_part2(input) as i64
}

type Map = Vec<Vec<u8>>;
type Point = (isize, isize);

fn parse_data(data: String) -> Map {
    data.lines().map(|l| l.bytes().collect_vec()).collect_vec()
}

fn walkable1(map: &Map, point: Point) -> Vec<Point> {
    let mut ret = Vec::new();

    const row_v: [isize; 4] = [-1, 1, 0, 0];
    const col_v: [isize; 4] = [0, 0, -1, 1];

    for i in 0..4 {
        let row = point.0 + row_v[i];
        let col = point.1 + col_v[i];

        if row >= 0
            && row < map.len() as isize
            && col >= 0
            && col < map[0].len() as isize
            && map[row as usize][col as usize] <= map[point.0 as usize][point.1 as usize] + 1
        {
            ret.push((row, col));
        }
    }

    ret
}

fn solve_part1(mut input: Map) -> i32 {
    let mut start = (0, 0);
    let mut end = (0, 0);
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            if input[i][j] == b'S' {
                start = (i as isize, j as isize);
                input[i][j] = b'a';
            }
            if input[i][j] == b'E' {
                end = (i as isize, j as isize);
                input[i][j] = b'z';
            }
        }
    }

    // BFS
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back((start, 0));
    visited.insert(start);

    let mut ret = 0;

    'outer: while !queue.is_empty() {
        let (current, steps) = queue.pop_front().unwrap();
        for node in walkable1(&input, current) {
            if node == end {
                ret = steps;
                break 'outer;
            }

            if !visited.contains(&node) {
                queue.push_back((node, steps + 1));
                visited.insert(node);
            }
        }
    }

    ret + 1
}

fn walkable2(map: &Map, point: Point) -> Vec<Point> {
    let mut ret = Vec::new();

    const row_v: [isize; 4] = [-1, 1, 0, 0];
    const col_v: [isize; 4] = [0, 0, -1, 1];

    for i in 0..4 {
        let row = point.0 + row_v[i];
        let col = point.1 + col_v[i];

        if row >= 0
            && row < map.len() as isize
            && col >= 0
            && col < map[0].len() as isize
            && map[row as usize][col as usize] >= map[point.0 as usize][point.1 as usize] - 1
        {
            ret.push((row, col));
        }
    }

    ret
}

fn solve_part2(mut input: Map) -> i32 {
    let mut end = (0, 0);
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            if input[i][j] == b'S' {
                input[i][j] = b'a';
            }
            if input[i][j] == b'E' {
                end = (i as isize, j as isize);
                input[i][j] = b'z';
            }
        }
    }

    // BFS
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back((end, 0));
    visited.insert(end);

    let mut ret = 0;

    'outer: while !queue.is_empty() {
        let (current, steps) = queue.pop_front().unwrap();
        for node in walkable2(&input, current) {
            if input[node.0 as usize][node.1 as usize] == b'a' {
                ret = steps;
                break 'outer;
            }

            if !visited.contains(&node) {
                queue.push_back((node, steps + 1));
                visited.insert(node);
            }
        }
    }

    ret + 1
}
