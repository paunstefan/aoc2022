use std::collections::HashSet;

use itertools::Itertools;

pub fn run_part1(path: &str) -> i64 {
    let input = parse_data(std::fs::read_to_string(path).unwrap());

    solve_part1(input) as i64
}

pub fn run_part2(path: &str) -> i64 {
    let input = parse_data(std::fs::read_to_string(path).unwrap());

    solve_part2(input) as i64
}

type Move = (char, i32);
type Position = (i32, i32);

fn parse_data(input: String) -> Vec<Move> {
    input
        .lines()
        .map(|line| {
            let mut elems = line.split_whitespace();
            let direction = elems.next().unwrap().chars().next().unwrap();
            let distance = elems.next().unwrap().parse::<i32>().unwrap();
            (direction, distance)
        })
        .collect_vec()
}

struct Rope {
    nodes: Vec<Position>,
    tail: usize,
    pub tvisited: HashSet<Position>,
}

impl Rope {
    fn new(n: usize) -> Self {
        let mut tvisited = HashSet::new();
        tvisited.insert((0, 0));
        Rope {
            nodes: vec![(0, 0); n],
            tail: n - 1,
            tvisited,
        }
    }

    fn ht_distance(&self, h: usize, t: usize) -> i32 {
        // Chebyshev distance
        i32::max(
            (self.nodes[h].0 - self.nodes[t].0).abs(),
            (self.nodes[h].1 - self.nodes[t].1).abs(),
        )
    }

    fn command(&mut self, direction: Move) {
        for _ in 0..direction.1 {
            let moves = match direction.0 {
                'L' => (-1, 0),
                'R' => (1, 0),
                'U' => (0, 1),
                'D' => (0, -1),
                _ => (0, 0),
            };

            // Move head
            self.nodes[0].0 += moves.0;
            self.nodes[0].1 += moves.1;

            for i in 0..self.tail {
                if self.ht_distance(i, i + 1) > 1 {
                    let h = i;
                    let t = i + 1;
                    // Adjust tail
                    // It will move 1 position in the direction of the head
                    self.nodes[t].1 += (self.nodes[h].1 - self.nodes[t].1).signum();
                    self.nodes[t].0 += (self.nodes[h].0 - self.nodes[t].0).signum();
                }
            }
            self.tvisited
                .insert((self.nodes[self.tail].0, self.nodes[self.tail].1));
        }
    }
}

fn solve_part1(input: Vec<Move>) -> i32 {
    let mut rope = Rope::new(2);
    for m in input {
        rope.command(m);
    }
    rope.tvisited.len() as i32
}

fn solve_part2(input: Vec<Move>) -> i32 {
    let mut rope = Rope::new(10);
    for m in input {
        rope.command(m);
    }
    rope.tvisited.len() as i32
}
