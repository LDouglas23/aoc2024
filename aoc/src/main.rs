#![allow(clippy::pedantic)]

use std::{
    fs::File,
    io::{self, BufRead},
};

// pub mod problem1;
// pub mod problem2;
// pub mod problem3;
// pub mod problem4;
// pub mod problem5;
pub mod problem6;

fn main() -> io::Result<()> {
    let f = File::open("inputs/6.txt")?;
    let lines: Vec<String> = io::BufReader::new(f).lines().map(|l| l.unwrap()).collect();

    let input = problem6::parse_lines(lines);
    // let solution = problem6::solution(input);
    let solution = problem6::solution_part_two(input);

    println!("Solution {:?}", solution);

    Ok(())
}
