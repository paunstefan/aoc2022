use itertools::Itertools;

pub fn run_part1(path: &str) -> i64 {
    let input = parse_data(std::fs::read_to_string(path).unwrap());
    solve_part1(input) as i64
}

pub fn run_part2(path: &str) -> i64 {
    let input = parse_data(std::fs::read_to_string(path).unwrap());

    solve_part2(input) as i64
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Node {
    Value(i32),
    List(Vec<Node>),
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Node::Value(x), Node::Value(y)) => x.partial_cmp(y),
            (Node::List(x), Node::List(y)) => x.partial_cmp(y),
            (Node::Value(_), Node::List(_)) => Node::List(vec![self.clone()]).partial_cmp(other),
            (Node::List(_), Node::Value(_)) => self.partial_cmp(&Node::List(vec![other.clone()])),
        }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn parse_data(data: String) -> Vec<(Node, Node)> {
    data.split("\n\n")
        .map(|s| s.replace(',', " ").replace('[', " [ ").replace(']', " ] "))
        .map(parse_pair)
        .collect_vec()
}

fn parse_pair(input: String) -> (Node, Node) {
    let lines = input
        .lines()
        .map(|l| l.split_whitespace().collect_vec())
        .collect_vec();

    let (n1, _) = parse_node(&lines[0], 0);
    let (n2, _) = parse_node(&lines[1], 0);
    //println!("{:?}\n{:?}\n", n1, n2);
    (n1, n2)
}

fn parse_node(input: &[&str], mut index: usize) -> (Node, usize) {
    if input[index] == "[" {
        let mut v = Vec::new();
        while input[index + 1] != "]" {
            let (inner, idx) = parse_node(input, index + 1);
            index = idx;
            v.push(inner);
        }
        (Node::List(v), index + 1)
    } else {
        let val = input[index].parse::<i32>().unwrap();

        (Node::Value(val), index)
    }
}

fn solve_part1(input: Vec<(Node, Node)>) -> usize {
    input
        .iter()
        .enumerate()
        .filter(|(_, (left, right))| left <= right)
        .map(|(i, _)| i + 1)
        .sum()
}

fn solve_part2(input: Vec<(Node, Node)>) -> usize {
    let mut all = Vec::new();
    let delimiter1 = Node::List(vec![Node::List(vec![Node::Value(2)])]);
    let delimiter2 = Node::List(vec![Node::List(vec![Node::Value(6)])]);
    all.push(delimiter1.clone());
    all.push(delimiter2.clone());

    for (n1, n2) in input {
        all.push(n1);
        all.push(n2);
    }

    all.sort();
    all.iter()
        .enumerate()
        .filter(|(_, n)| **n == delimiter1 || **n == delimiter2)
        .map(|(i, _)| i + 1)
        .product()
}
