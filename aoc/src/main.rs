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
// pub mod problem6;
pub mod problem7;

fn main() -> io::Result<()> {
    let f = File::open("inputs/7.txt")?;
    let lines: Vec<String> = io::BufReader::new(f).lines().map(|l| l.unwrap()).collect();

    let input = problem7::Input::from_lines(&lines);
    // let solution = problem7::solution(input);
    let solution = problem7::solution_part_two(input);

    println!("Solution {:?}", solution);

    Ok(())
}
