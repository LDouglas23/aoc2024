#![allow(clippy::pedantic)]

use std::{
    fs::File,
    io::{self, BufRead},
};

pub mod common;

// pub mod problem1;
// pub mod problem2;
// pub mod problem3;
// pub mod problem4;
// pub mod problem5;
// pub mod problem6;
// pub mod problem7;
// pub mod problem8;
// pub mod problem9;
// pub mod problem10;
// pub mod problem11;
pub mod problem12;

fn main() -> io::Result<()> {
    let f = File::open("inputs/12.txt")?;
    let lines: Vec<String> = io::BufReader::new(f).lines().map(|l| l.unwrap()).collect();

    let input = problem12::Input::from(lines);
    let solution = problem12::solution(input);
    // let solution = problem12::solution_part_two(input);

    println!("Solution {:?}", solution);

    Ok(())
}
