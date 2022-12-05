pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;

use std::{fmt::Debug, str::FromStr};

pub fn parse_lines<F, B>(inp: &str) -> B
where
    F: FromStr,
    F::Err: Debug,
    B: FromIterator<F>,
{
    inp.lines().map(|line| line.parse().unwrap()).collect()
}
