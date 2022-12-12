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

use std::{fmt::Debug, str::FromStr};

extern crate derive_more;
use derive_more::From;

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

#[derive(Debug, Clone, From)]
pub struct VecP<T>(Vec<T>);

impl<T: std::str::FromStr> FromStr for VecP<T> {
    type Err = T::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.split(',')
                .map(|v| v.trim().parse())
                .collect::<Result<_, Self::Err>>()?,
        ))
    }
}
