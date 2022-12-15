pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;

use std::{fmt::Debug, str::FromStr};

pub fn parse_lines<F: FromStr, B: FromIterator<F>>(inp: &str) -> B
where
    F::Err: Debug,
{
    parse_split(inp, "\n")
}

pub fn parse_split<F: FromStr, B: FromIterator<F>>(inp: &str, p: &str) -> B
where
    F::Err: Debug,
{
    inp.split(p).map(|line| line.parse().unwrap()).collect()
}
