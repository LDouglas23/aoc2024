#![allow(clippy::pedantic)]

use std::{
    fs::File,
    io::{self, BufRead},
};

// pub mod problem1;
// pub mod problem2;
// pub mod problem3;
// pub mod problem4;
pub mod problem5;

fn main() -> io::Result<()> {
    let f = File::open("inputs/5.txt")?;
    let lines: Vec<String> = io::BufReader::new(f).lines().map(|l| l.unwrap()).collect();

    let input = problem5::parse_lines(lines);
    // let solution = problem5::solution(input);
    let solution = problem5::solution_part_two(input);

    println!("Solution {:?}", solution);

    Ok(())
}
