use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct Equation {
    target: usize,
    numbers: Vec<usize>,
}

impl From<&String> for Equation {
    fn from(value: &String) -> Self {
        let parts = value.split(": ").map(String::from).collect::<Vec<String>>();

        assert!(parts.len() == 2);

        Self {
            target: parts[0].parse().expect("failed to parse target"),
            numbers: parts[1]
                .split(" ")
                .map(|n| n.parse().expect("failed to parse number"))
                .collect(),
        }
    }
}

#[derive(Debug, Clone)]
struct Node {
    val: usize,
    left: Option<NodeRef>,
    right: Option<NodeRef>,
}

type NodeRef = Box<Node>;

impl Node {
    fn new(value: usize) -> Self {
        Self {
            val: value,
            left: None,
            right: None,
        }
    }

    fn extend(&mut self, value: usize) {
        let mut queue: VecDeque<&mut Node> = VecDeque::new();

        queue.push_front(self);

        loop {
            let Node {
                ref mut left,
                ref mut right,
                val,
                ..
            } = queue.pop_back().unwrap();

            match left {
                Some(node) => {
                    queue.push_front(node);
                }
                None => {
                    *left = Some(Box::new(Node::new(*val + value)));
                }
            }

            match right {
                Some(node) => {
                    queue.push_front(node);
                }
                None => {
                    *right = Some(Box::new(Node::new(*val * value)));
                }
            }

            if queue.is_empty() {
                return;
            }
        }
    }

    fn get_leaves(&self) -> Vec<&Self> {
        let mut queue: Vec<&Node> = vec![self];
        let mut leaves: Vec<&Node> = vec![];

        while !queue.is_empty() {
            let node = queue.pop().unwrap();

            match &node.left {
                Some(left) => {
                    queue.push(left);
                    match &node.right {
                        Some(right) => queue.push(right),
                        None => {}
                    }
                }
                None => leaves.push(&node),
            }
        }

        leaves
    }
}

#[derive(Debug, Clone)]
struct NodeExtended {
    val: usize,
    left: Option<NodeExtendedRef>,
    middle: Option<NodeExtendedRef>,
    right: Option<NodeExtendedRef>,
}

type NodeExtendedRef = Box<NodeExtended>;

impl NodeExtended {
    fn new(value: usize) -> Self {
        Self {
            val: value,
            left: None,
            middle: None,
            right: None,
        }
    }

    fn extend(&mut self, value: usize) {
        let mut queue: VecDeque<&mut NodeExtended> = VecDeque::new();

        queue.push_front(self);

        loop {
            let NodeExtended {
                ref mut left,
                ref mut middle,
                ref mut right,
                val,
                ..
            } = queue.pop_back().unwrap();

            match left {
                Some(node) => {
                    queue.push_front(node);
                }
                None => {
                    *left = Some(Box::new(NodeExtended::new(*val + value)));
                }
            }

            match middle {
                Some(node) => {
                    queue.push_front(node);
                }
                None => {
                    *middle = Some(Box::new(NodeExtended::new(val.concat(value))));
                }
            }

            match right {
                Some(node) => {
                    queue.push_front(node);
                }
                None => {
                    *right = Some(Box::new(NodeExtended::new(*val * value)));
                }
            }

            if queue.is_empty() {
                return;
            }
        }
    }

    fn get_leaves(&self) -> Vec<&Self> {
        let mut queue: Vec<&Self> = vec![self];
        let mut leaves: Vec<&Self> = vec![];

        while !queue.is_empty() {
            let node = queue.pop().unwrap();

            match &node.left {
                Some(left) => {
                    queue.push(left);
                    match &node.middle {
                        Some(middle) => {
                            queue.push(middle);
                            match &node.right {
                                Some(right) => queue.push(right),
                                None => {}
                            }
                        }
                        None => {}
                    }
                }
                None => leaves.push(&node),
            }
        }

        leaves
    }
}

impl Equation {
    fn has_solution(&self) -> bool {
        let mut tree = Node::new(self.numbers[0]);

        for i in 1..self.numbers.len() {
            tree.extend(self.numbers[i]);
        }

        tree.get_leaves()
            .iter()
            .any(|&leaf| leaf.val == self.target)
    }

    fn has_solution_extended(&self) -> bool {
        let mut tree = NodeExtended::new(self.numbers[0]);

        for i in 1..self.numbers.len() {
            tree.extend(self.numbers[i]);
        }

        tree.get_leaves()
            .iter()
            .any(|&leaf| leaf.val == self.target)
    }
}

pub struct Input {
    equations: Vec<Equation>,
}

impl Input {
    pub fn from_lines(lines: &[String]) -> Self {
        Self {
            equations: lines.iter().map(|line| Equation::from(line)).collect(),
        }
    }
}

pub fn solution(input: Input) -> usize {
    input
        .equations
        .iter()
        .filter(|&eq| eq.has_solution())
        .map(|eq| eq.target)
        .sum()
}

trait Concatentate<T> {
    fn concat(&self, other: T) -> T;
}

impl Concatentate<usize> for usize {
    fn concat(&self, other: usize) -> usize {
        let mag = (other as f64).log10() as u32 + 1;

        (self * (10 as usize).pow(mag)) + other
    }
}

pub fn solution_part_two(input: Input) -> usize {
    input
        .equations
        .iter()
        .filter(|&eq| eq.has_solution_extended())
        .map(|eq| eq.target)
        .sum()
}

#[cfg(test)]
mod test {
    use super::{Concatentate, Input};

    const EXAMPLE: &'static str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

    #[test]
    pub fn test() {
        let input = Input::from_lines(&EXAMPLE.lines().map(String::from).collect::<Vec<String>>());

        assert_eq!(super::solution(input), 3749);
    }

    #[test]
    pub fn test2() {
        let input = Input::from_lines(&EXAMPLE.lines().map(String::from).collect::<Vec<String>>());

        assert_eq!(super::solution_part_two(input), 11387);
    }

    #[test]
    pub fn concat_test() {
        assert_eq!((15 as usize).concat(6), 156);
        assert_eq!((8 as usize).concat(6), 86);
        assert_eq!((10 as usize).concat(0), 100);
        assert_eq!((14 as usize).concat(20), 1420);
    }
}
