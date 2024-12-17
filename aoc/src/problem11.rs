use std::vec;

#[derive(Debug)]
pub struct Input {
    stones: Vec<usize>,
}

impl From<Vec<String>> for Input {
    fn from(value: Vec<String>) -> Self {
        Self {
            stones: value[0]
                .split(" ")
                .map(|s| s.parse::<usize>().unwrap())
                .collect(),
        }
    }
}

fn solve(input: Input, num_blinks: usize) -> usize {
    let mut stones = input.stones;

    for i in 0..num_blinks {
        println!("Blink {}", i);
        println!("{}", stones.len());
        let mut next: Vec<usize> = vec![];

        for stone in stones {
            if stone == 0 {
                next.push(1);
            } else if stone.ilog10() % 2 == 1 {
                let length = stone.ilog10() as usize + 1;
                let mut left = stone;
                let mut right = stone;

                for _ in 0..(length / 2) {
                    left = left / 10;
                }

                right = right - left * (10 as usize).pow(length as u32 / 2);

                next.push(left);
                next.push(right);
            } else {
                next.push(stone * 2024)
            }
        }

        stones = next;
    }
    stones.len()
}

pub fn solution(input: Input) -> usize {
    solve(input, 25)
}

pub fn solution_part_two(input: Input) -> usize {
    let mut result = 0;

    for stone in input.stones {
        result += blink(stone, 74);
    }

    result
}

fn blink(stone: usize, count: usize) -> usize {
    let next = step(stone);

    if count == 0 {
        next.len()
    } else {
        if next.len() == 1 {
            blink(next[0], count - 1)
        } else {
            blink(next[0], count - 1) + blink(next[1], count - 1)
        }
    }
}

fn step(stone: usize) -> Vec<usize> {
    if stone == 0 {
        return vec![1];
    } else if stone.ilog10() % 2 == 1 {
        let length = stone.ilog10() as usize + 1;
        let mut left = stone;
        let mut right = stone;

        for _ in 0..(length / 2) {
            left = left / 10;
        }

        right = right - left * (10 as usize).pow(length as u32 / 2);

        return vec![left, right];
    } else {
        return vec![stone * 2024];
    }
}

#[cfg(test)]
mod test {
    use super::{solution, solution_part_two, Input};

    #[test]
    pub fn test() {
        let stones = "125 17";

        let input = Input::from(vec![stones.to_owned()]);

        assert_eq!(solution(input), 55312);
    }

    #[test]
    pub fn test2() {
        let stones = "125 17";
        let input = Input::from(vec![stones.to_owned()]);

        assert_eq!(solution_part_two(input), 55312);
    }
}
