#![allow(clippy::pedantic)]

use std::{
    fs::File,
    io::{self, BufRead},
};

pub mod problem1;

fn main() -> io::Result<()> {
    let f = File::open("inputs/1.txt")?;
    let lines: Vec<String> = io::BufReader::new(f).lines().map(|l| l.unwrap()).collect();

    let input = problem1::parse_lines(lines);
    // let solution = problem1::solution(input);
    let solution = problem1::solution_part_two(input);

    println!("Solution {:?}", solution);

    Ok(())
}
