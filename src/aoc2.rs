use itertools::Itertools;

pub fn run_part1(path: &str) -> i64 {
    let data = parse_data(std::fs::read_to_string(path).unwrap());

    solve_part1(data) as i64
}

pub fn run_part2(path: &str) -> i64 {
    let data = parse_data(std::fs::read_to_string(path).unwrap());

    solve_part2(data) as i64
}

fn parse_data(data: String) -> Vec<(u8, u8)> {
    data.lines()
        .map(|l| {
            let a = l.as_bytes().first().unwrap();
            let b = l.as_bytes().get(2).unwrap();

            (a - b'A', b - b'X')
        })
        .collect_vec()
}

// Shapes are ROCK(0), PAPER(1), SCISSORS(2)
// Each shape is beaten by (shape + 1) % 3
fn win_score(game: &(u8, u8)) -> i32 {
    if game.0 == game.1 {
        return 3; // Tie
    }
    if (game.0 + 1) % 3 == game.1 {
        return 6; // Win
    }
    0
}

fn shape_score(shape: u8) -> i32 {
    (shape + 1) as i32
}

fn solve_part1(games: Vec<(u8, u8)>) -> i32 {
    let mut sum = 0;
    for game in games {
        sum += win_score(&game) + shape_score(game.1);
    }
    sum
}

const LOSE: u8 = 0;
const TIE: u8 = 1;
const WIN: u8 = 2;

fn solve_part2(games: Vec<(u8, u8)>) -> i32 {
    let mut sum = 0;
    for game in games {
        let shape = match game.1 {
            // Shape is beaten by shape - 1, wrapping 0 to 2
            LOSE => game.0.checked_sub(1).unwrap_or(2),
            TIE => game.0,
            WIN => (game.0 + 1) % 3,
            _ => panic!("Invalid shape)"),
        };
        let actual_game = (game.0, shape);
        sum += win_score(&actual_game) + shape_score(actual_game.1);
    }
    sum
}
