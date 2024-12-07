#![allow(clippy::pedantic)]

use std::{
    fs::File,
    io::{self, BufRead},
};

// pub mod problem1;
// pub mod problem2;
pub mod problem3;

fn main() -> io::Result<()> {
    let f = File::open("inputs/3.txt")?;
    let lines: Vec<String> = io::BufReader::new(f).lines().map(|l| l.unwrap()).collect();

    let input = problem3::parse_lines(lines);
    // let solution = problem3::solution(input);
    let solution = problem3::solution_part_two(input);

    println!("Solution {:?}", solution);

    Ok(())
}
