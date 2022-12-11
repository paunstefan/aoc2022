use std::collections::VecDeque;

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
struct MonkeyArray {
    monkeys: Vec<Monkey>,
    modulo: u64,
}

impl MonkeyArray {
    fn run_round(&mut self, relief: u64) {
        for i in 0..self.monkeys.len() {
            while let Some((worry, next)) = self.monkeys[i].inspect(relief, self.modulo) {
                self.monkeys[next].add_item(worry);
            }
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    test: (u64, usize, usize),
    inspected: u64,
}

impl Monkey {
    fn inspect(&mut self, relief: u64, modulo: u64) -> Option<(u64, usize)> {
        if self.items.is_empty() {
            None
        } else {
            let item = self.items.pop_front().unwrap();
            let worry = self.operation.calculate(item);
            let bored = (worry / relief) % modulo;

            let next_monkey = if bored % self.test.0 == 0 {
                self.test.1
            } else {
                self.test.2
            };
            self.inspected += 1;

            Some((bored, next_monkey))
        }
    }

    fn add_item(&mut self, item: u64) {
        self.items.push_back(item)
    }
}

#[derive(Debug)]
struct Operation {
    operator: Operator,
    operands: (Operand, Operand),
}

impl Operation {
    fn calculate(&self, old: u64) -> u64 {
        let op1 = match self.operands.0 {
            Operand::Old => old,
            Operand::Num(n) => n,
        };
        let op2 = match self.operands.1 {
            Operand::Old => old,
            Operand::Num(n) => n,
        };

        match self.operator {
            Operator::Add => op1 + op2,
            Operator::Mult => op1 * op2,
        }
    }
}

#[derive(Debug)]
enum Operand {
    Old,
    Num(u64),
}

#[derive(Debug)]
enum Operator {
    Add,
    Mult,
}

fn parse_data(input: String) -> MonkeyArray {
    let monkeys = input.split("\n\n").map(parse_monkey).collect_vec();
    let modulo = monkeys.iter().map(|m| m.test.0).product();
    MonkeyArray { monkeys, modulo }
}

fn parse_monkey(input: &str) -> Monkey {
    let mut lines = input.lines();
    // parse starting items
    let items = lines
        .nth(1)
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .split(',')
        .map(|n| n.strip_prefix(' ').unwrap().parse::<u64>().unwrap())
        .collect_vec()
        .into();

    // parse operation
    let parts = lines
        .next()
        .unwrap()
        .split('=')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .collect_vec();
    let operator = match parts[1] {
        "+" => Operator::Add,
        "*" => Operator::Mult,
        _ => panic!("Invalid operator"),
    };

    let operands = match (parts[0], parts[2]) {
        ("old", "old") => (Operand::Old, Operand::Old),
        ("old", _) => (Operand::Old, Operand::Num(parts[2].parse().unwrap())),
        (_, "old") => (Operand::Num(parts[0].parse().unwrap()), Operand::Old),
        (_, _) => (
            Operand::Num(parts[0].parse().unwrap()),
            Operand::Num(parts[2].parse().unwrap()),
        ),
    };

    let operation = Operation { operator, operands };

    // parse test
    let num = lines
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<u64>()
        .unwrap();

    let t = lines
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();

    let f = lines
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();

    Monkey {
        items,
        operation,
        test: (num, t, f),
        inspected: 0,
    }
}

fn solve_part1(mut input: MonkeyArray) -> u64 {
    for _ in 0..20 {
        input.run_round(3);
    }

    input
        .monkeys
        .iter()
        .map(|m| m.inspected)
        .sorted()
        .rev()
        .take(2)
        .product()
}
fn solve_part2(mut input: MonkeyArray) -> u64 {
    for _ in 0..10000 {
        input.run_round(1);
    }

    input
        .monkeys
        .iter()
        .map(|m| m.inspected)
        .sorted()
        .rev()
        .take(2)
        .product()
}
