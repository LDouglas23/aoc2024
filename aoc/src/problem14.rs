use std::{collections::HashMap, thread::sleep, time::Duration};

use crate::common::Vector2di;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref ROBOT_RE: Regex = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
}

#[derive(Debug, Clone, Copy)]
struct Robot {
    position: Vector2di,
    velocity: Vector2di,
}

impl Robot {
    fn tick(&mut self, bounds: Vector2di) {
        self.position.x = (self.position.x + self.velocity.x).rem_euclid(bounds.x);
        self.position.y = (self.position.y + self.velocity.y).rem_euclid(bounds.y);
    }
}

impl From<&String> for Robot {
    fn from(value: &String) -> Self {
        ROBOT_RE
            .captures_iter(value)
            .map(|caps| {
                let (_, [p_x, p_y, v_x, v_y]) = caps.extract();

                Self {
                    position: Vector2di::new(p_x.parse().unwrap(), p_y.parse().unwrap()),
                    velocity: Vector2di::new(v_x.parse().unwrap(), v_y.parse().unwrap()),
                }
            })
            .collect::<Vec<Robot>>()[0]
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    robots: Vec<Robot>,
    bounds: Vector2di,
}

impl From<Vec<String>> for Input {
    fn from(value: Vec<String>) -> Self {
        Self {
            robots: value.iter().map(Robot::from).collect(),
            bounds: Vector2di::new(101, 103),
        }
    }
}

impl Input {
    fn simulate_motion(&mut self, num_ticks: usize) {
        for _ in 0..num_ticks {
            for robot in &mut self.robots {
                robot.tick(self.bounds);
            }
        }
    }
}

fn get_safety_factor(robots: Vec<Robot>, bounds: Vector2di) -> usize {
    let vertical_boundary = bounds.x / 2;
    let horizontal_boundary = bounds.y / 2;

    let q1 = &robots
        .iter()
        .filter(|&r| r.position.x < vertical_boundary && r.position.y < horizontal_boundary)
        .count();

    let q2 = &robots
        .iter()
        .filter(|&r| r.position.x > vertical_boundary && r.position.y < horizontal_boundary)
        .count();

    let q3 = &robots
        .iter()
        .filter(|&r| r.position.x < vertical_boundary && r.position.y > horizontal_boundary)
        .count();

    let q4 = &robots
        .iter()
        .filter(|&r| r.position.x > vertical_boundary && r.position.y > horizontal_boundary)
        .count();

    q1 * q2 * q3 * q4
}

pub fn solution(input: Input) -> usize {
    let mut input = input.clone();

    input.simulate_motion(100);

    get_safety_factor(input.robots, input.bounds)
}

fn get_map(robots: &[Robot]) -> HashMap<Vector2di, usize> {
    let mut map = HashMap::new();

    for robot in robots {
        if let Some(v) = map.get(&robot.position) {
            map.insert(robot.position, v + 1);
        } else {
            map.insert(robot.position, 1);
        }
    }

    map
}

fn display_map(map: &HashMap<Vector2di, usize>, bounds: Vector2di) {
    for j in 0..bounds.y {
        for i in 0..bounds.x {
            // if i == bounds.x / 2 || j == bounds.y / 2 {
            // print!(" ");
            // continue;
            // }
            //
            if let Some(v) = map.get(&Vector2di::new(i, j)) {
                print!("{}", v);
            } else {
                print!(".");
            }
        }

        println!();
    }
}

pub fn solution_part_two(input: Input) {
    let mut input = input.clone();

    for _ in 0..10000 {
        input.simulate_motion(1);

        if input
            .robots
            .iter()
            .any(|r| r.position.x == 50 && r.position.y == 0)
        {
            let map = get_map(&input.robots);
            display_map(&map, input.bounds);
            break;
        }
    }

    println!("No result after 10000 ticks");
}

#[cfg(test)]
mod test {
    use crate::common::Vector2di;

    use super::{solution, Input, Robot};

    const EXAMPLE: &'static str = "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";

    #[test]
    fn test() {
        let input = Input {
            robots: EXAMPLE
                .lines()
                .map(|s| Robot::from(&s.to_owned()))
                .collect(),
            bounds: Vector2di::new(11, 7),
        };

        assert_eq!(solution(input), 12);
    }
}
