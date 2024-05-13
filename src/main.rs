use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

mod day1;
mod day10;
mod day11;
mod day15;
mod day16;
mod day17;
mod day19;
mod day2;
mod day20;
mod day21;
mod day24;
mod day25;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

pub fn read_input_file<P>(fp: P) -> String
where
    P: AsRef<Path>,
{
    //! Given a file path, returns the entire file contents as a String.
    let file = File::open(fp).unwrap();
    let mut reader = BufReader::new(file);
    let mut buffer_string = String::new();
    reader
        .read_to_string(&mut buffer_string)
        .expect("Could not read file to string");
    buffer_string
}

///trait representing how to solve the days challenge for the advent calendar.
///Obviously, part1 is for part1 and part2 is for part2.
/// The trait methods do not return anything, the answer to the problem
/// should simply be printed.
trait SolveAdvent {
    ///How to solve part1 of the days puzzle.
    fn solve_part1(path_to_file: &str);
    ///How to solve part2 of the days puzzle.
    fn solve_part2(path_to_file: &str);
}

fn main() {
    day17::Day17::solve_part1("input.txt");
    day17::Day17::solve_part2("test.txt");
}
