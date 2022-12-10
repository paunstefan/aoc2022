use itertools::Itertools;
use std::sync::mpsc;

pub fn run_part1(path: &str) -> i64 {
    let input = parse_data(std::fs::read_to_string(path).unwrap());

    solve_part1(input) as i64
}

pub fn run_part2(path: &str) -> i64 {
    let input = parse_data(std::fs::read_to_string(path).unwrap());

    solve_part2(input) as i64
}

enum Instruction {
    Add(i32),
    Noop,
}

struct CPU {
    pc: usize,
    clock: usize,
    x: i32,
    program: Vec<Instruction>,
    breakpoints: Option<Vec<usize>>,
    mq: Option<mpsc::Sender<i32>>,
}

impl CPU {
    fn new(program: Vec<Instruction>) -> Self {
        CPU {
            pc: 0,
            clock: 1,
            x: 1,
            program,
            breakpoints: None,
            mq: None,
        }
    }

    fn run(&mut self) {
        let mut pending: Option<i32> = None;
        loop {
            if self.pc >= self.program.len() {
                break;
            }

            self.draw();

            if pending.is_none() {
                match self.program[self.pc] {
                    Instruction::Add(n) => {
                        self.pc += 1;
                        pending = Some(n);
                    }
                    Instruction::Noop => self.pc += 1,
                }
            } else {
                let n = pending.unwrap();
                self.x += n;
                pending = None;
            }

            self.clock += 1;

            if let Some(ref bk) = self.breakpoints {
                if bk.contains(&self.clock) {
                    if let Some(ref sender) = self.mq {
                        sender.send(self.x * self.clock as i32).unwrap();
                    }
                }
            }
        }
    }

    fn set_breakpoints(&mut self, bk: Vec<usize>) -> mpsc::Receiver<i32> {
        let (sender, receiver) = mpsc::channel();
        self.breakpoints = Some(bk);
        self.mq = Some(sender);
        receiver
    }

    fn draw(&self) {
        //println!("[{} {}]", self.x, self.clock);
        if (self.x - 1..self.x + 2).contains(&((self.clock - 1) as i32 % 40)) {
            print!("#");
        } else {
            print!(".");
        }

        if self.clock % 40 == 0 {
            println!();
        }
    }
}

fn parse_instruction(input: &str) -> Instruction {
    if input.starts_with("addx") {
        let instr = input
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse::<i32>()
            .unwrap();
        Instruction::Add(instr)
    } else {
        Instruction::Noop
    }
}

fn parse_data(input: String) -> Vec<Instruction> {
    input.lines().map(parse_instruction).collect_vec()
}

fn solve_part1(input: Vec<Instruction>) -> i32 {
    let mut cpu = CPU::new(input);
    let rx = cpu.set_breakpoints(vec![20, 60, 100, 140, 180, 220]);

    cpu.run();

    let mut sum = 0;

    while let Ok(x) = rx.try_recv() {
        sum += x;
    }

    sum
}
fn solve_part2(input: Vec<Instruction>) -> i32 {
    let mut cpu = CPU::new(input);
    cpu.run();
    0
}
