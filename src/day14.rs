use crate::*;

use itertools::Itertools;
use parse_display::FromStr;
use std::collections::HashSet;

extern crate derive_more;
use derive_more::From;

#[derive(Debug, Clone, From)]
pub struct VecP<T>(Vec<T>);

impl<T: std::str::FromStr> FromStr for VecP<T> {
    type Err = T::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.split("->")
                .map(|v| v.trim().parse())
                .collect::<Result<_, Self::Err>>()?,
        ))
    }
}

#[derive(Debug, FromStr, Hash, Clone, Copy, Eq, PartialEq)]
#[display("{0},{1}")]
pub struct Point(usize, usize);

type Input = HashSet<Point>;

pub fn parse_input(input: &str) -> Input {
    let lines = parse_lines::<VecP<Point>, Vec<_>>(input);
    let mut points = HashSet::new();
    for path in lines.iter() {
        for (p0, p1) in path.0.iter().tuple_windows() {
            for i in p0.0.min(p1.0)..=p0.0.max(p1.0) {
                points.insert(Point(i, p0.1));
            }
            for i in p0.1.min(p1.1)..=p0.1.max(p1.1) {
                points.insert(Point(p0.0, i));
            }
        }
    }
    points
}

pub fn part1(inp: &Input) -> isize {
    let bottom = inp.iter().map(|p| p.1).max().unwrap();
    let mut filled = inp.clone();
    let mut count = 0;
    'outer: loop {
        let mut dropping = Point(500, 0);
        'inner: loop {
            if dropping.1 > bottom {
                break 'outer;
            }
            if !filled.contains(&Point(dropping.0, dropping.1 + 1)) {
                dropping = Point(dropping.0, dropping.1 + 1)
            } else if !filled.contains(&Point(dropping.0 - 1, dropping.1 + 1)) {
                dropping = Point(dropping.0 - 1, dropping.1 + 1)
            } else if !filled.contains(&Point(dropping.0 + 1, dropping.1 + 1)) {
                dropping = Point(dropping.0 + 1, dropping.1 + 1)
            } else {
                filled.insert(dropping);
                count += 1;
                break 'inner;
            }
        }
    }
    count
}

pub fn part2(inp: &Input) -> isize {
    let bottom = inp.iter().map(|p| p.1).max().unwrap() + 1;
    let mut filled = inp.clone();
    let mut count = 0;
    loop {
        let mut dropping = Point(500, 0);
        loop {
            if dropping.1 == bottom {
                break;
            }
            if !filled.contains(&Point(dropping.0, dropping.1 + 1)) {
                dropping = Point(dropping.0, dropping.1 + 1)
            } else if !filled.contains(&Point(dropping.0 - 1, dropping.1 + 1)) {
                dropping = Point(dropping.0 - 1, dropping.1 + 1)
            } else if !filled.contains(&Point(dropping.0 + 1, dropping.1 + 1)) {
                dropping = Point(dropping.0 + 1, dropping.1 + 1)
            } else {
                break;
            }
        }
        filled.insert(dropping);
        count += 1;
        if dropping == Point(500, 0) {
            break;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const EXAMPLE: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(EXAMPLE)), 24)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(EXAMPLE)), 93)
    }
}
