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
// pub mod problem12;
// pub mod problem13;
// pub mod problem14;
pub mod problem15;

fn main() -> io::Result<()> {
    let f = File::open("inputs/15.txt")?;
    let lines: Vec<String> = io::BufReader::new(f).lines().map(|l| l.unwrap()).collect();

    let input = problem15::Input::from(lines);
    let solution = problem15::solution(input);
    // let solution = problem14::solution_part_two(input);

    println!("Solution {:?}", solution);

    Ok(())
}
