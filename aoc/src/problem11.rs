use std::{collections::HashMap, vec};

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

type Length = usize;
type Stone = usize;

/*
    The caches store the final length that a given stone adds to the sequence after the specified number of steps.

    For example, depth_cache[2] stores the map for depth 3, so each key in the map has a value that indicates the length by which the sequence will increase when that key is observed with 3 steps remaining.
*/

type Cache = HashMap<Stone, Length>;
type Caches = Vec<Cache>;

fn expand(stone: Stone, depth: usize, caches: &mut Caches) -> Length {
    let cache = &caches[depth];

    match cache.get(&stone) {
        Some(length) => *length,
        None => {
            let next = step(stone);

            let result = if depth == 0 {
                next.len()
            } else {
                if next.len() == 1 {
                    expand(next[0], depth - 1, caches)
                } else {
                    expand(next[0], depth - 1, caches) + expand(next[1], depth - 1, caches)
                }
            };

            caches[depth].insert(stone, result);
            return result;
        }
    }
}

fn step(stone: Stone) -> Vec<Stone> {
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

pub fn solution_part_two(input: Input) -> usize {
    let mut caches: Caches = vec![];
    let mut result = 0;

    for i in 0..75 {
        caches.push(HashMap::new());

        let mut lengths = 0;

        for stone in &input.stones {
            lengths += expand(*stone, i, &mut caches);
        }

        result = lengths;
    }
    result
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
