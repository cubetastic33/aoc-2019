// Written by Aravindan. My solutions in rust for Advent of Code 2019
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

pub mod day_1;

pub fn read_lines_from_file(filename: impl AsRef<Path>) -> Vec<usize> {
    let file = File::open(filename).expect("File not found");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line").parse().unwrap())
        .collect()
}
