use std::collections::VecDeque;

use crate::common::{CellContents, Grid, Vector2D};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Step {
    x: usize,
    y: usize,
    height: usize,
}

impl CellContents for Step {
    fn from(x: usize, y: usize, c: &char) -> Self {
        Self {
            x,
            y,
            height: c.to_digit(10).unwrap() as usize,
        }
    }
}

struct TopoMap {
    grid: Grid<Step>,
}

pub struct Input {
    map: TopoMap,
}

impl From<Vec<String>> for Input {
    fn from(value: Vec<String>) -> Self {
        Self {
            map: TopoMap {
                grid: Grid::new(
                    value
                        .iter()
                        .map(|s| s.chars().collect::<Vec<char>>())
                        .collect(),
                ),
            },
        }
    }
}

pub fn solution(input: Input) -> usize {
    let mut result = 0;

    let grid = input.map.grid;
    let trailheads: Vec<Step> = grid
        .clone()
        .into_iter()
        .filter_map(|cell| {
            if cell.contents().height == 0 {
                Some(cell.contents())
            } else {
                None
            }
        })
        .collect();

    for trailhead in trailheads {
        let mut queue: VecDeque<Step> = VecDeque::new();
        let mut visited_peaks: Vec<Step> = vec![];
        let mut score = 0;

        queue.push_back(trailhead);

        while !queue.is_empty() {
            let step = queue.pop_back().unwrap();

            let neighbours = grid.neighbours(Vector2D::new(step.x as i32, step.y as i32));

            for n in neighbours {
                if step.height == 8 && n.height == 9 && !visited_peaks.contains(&n) {
                    visited_peaks.push(n);
                    score += 1;
                } else if n.height == step.height + 1 {
                    queue.push_back(n);
                }
            }
        }

        println!(
            "Trailhead at {}, {} has score {}",
            trailhead.x, trailhead.y, score
        );
        result += score;
    }

    result
}

pub fn solution_part_two(input: Input) -> usize {
    let mut result = 0;

    let grid = input.map.grid;
    let trailheads: Vec<Step> = grid
        .clone()
        .into_iter()
        .filter_map(|cell| {
            if cell.contents().height == 0 {
                Some(cell.contents())
            } else {
                None
            }
        })
        .collect();

    for trailhead in trailheads {
        let mut rating = 0;
        let mut queue: VecDeque<Step> = VecDeque::new();

        queue.push_back(trailhead);

        while !queue.is_empty() {
            let step = queue.pop_back().unwrap();

            let neighbours = grid.neighbours(Vector2D::new(step.x as i32, step.y as i32));

            for n in neighbours {
                if step.height == 8 && n.height == 9 {
                    rating += 1;
                } else if n.height == step.height + 1 {
                    queue.push_back(n);
                }
            }
        }

        println!(
            "Trailhead at {}, {} has rating {}",
            trailhead.x, trailhead.y, rating
        );

        result += rating;
    }

    result
}

#[cfg(test)]
mod test {
    use crate::problem10::solution_part_two;

    use super::{solution, Input};

    const EXAMPLE: &'static str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    pub fn test() {
        let input = Input::from(EXAMPLE.lines().map(String::from).collect::<Vec<String>>());

        assert_eq!(solution(input), 36);
    }

    #[test]
    pub fn test2() {
        let input = Input::from(EXAMPLE.lines().map(String::from).collect::<Vec<String>>());

        assert_eq!(solution_part_two(input), 81);
    }
}
