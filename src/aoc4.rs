use itertools::Itertools;

type Interval = (i32, i32);

pub fn run_part1(path: &str) -> i64 {
    let input = parse_data(std::fs::read_to_string(path).unwrap());
    solve_part1(input) as i64
}

pub fn run_part2(path: &str) -> i64 {
    let input = parse_data(std::fs::read_to_string(path).unwrap());

    solve_part2(input) as i64
}

fn parse_data(data: String) -> Vec<[Interval; 2]> {
    data.lines()
        .map(|l| l.split(','))
        .map(|mut a| (a.next().unwrap(), a.next().unwrap()))
        .map(|(a, b)| {
            let mut first = a.split('-');
            let mut second = b.split('-');
            [
                (
                    first.next().unwrap().parse::<i32>().unwrap(),
                    first.next().unwrap().parse::<i32>().unwrap(),
                ),
                (
                    second.next().unwrap().parse::<i32>().unwrap(),
                    second.next().unwrap().parse::<i32>().unwrap(),
                ),
            ]
        })
        .collect_vec()
}

fn check_overlap1(intervals: &[Interval; 2]) -> bool {
    intervals[0].0 <= intervals[1].0 && intervals[0].1 >= intervals[1].1
        || intervals[1].0 <= intervals[0].0 && intervals[1].1 >= intervals[0].1
}

fn solve_part1(input: Vec<[Interval; 2]>) -> i32 {
    input
        .iter()
        .filter(|i| check_overlap1(i))
        .count()
        .try_into()
        .unwrap()
}

fn check_overlap2(intervals: &[Interval; 2]) -> bool {
    intervals[0].1 < intervals[1].0 || intervals[0].0 > intervals[1].1
}

fn solve_part2(input: Vec<[Interval; 2]>) -> i32 {
    input
        .iter()
        .filter(|i| !check_overlap2(i))
        .count()
        .try_into()
        .unwrap()
}
