use crate::*;

use itertools::Itertools;

type Input = Vec<Vec<isize>>;

pub fn parse_input(input: &str) -> Input {
    input
        .split("\n\n")
        .map(|group| parse_lines(group))
        .collect()
}

pub fn part1(inp: &Input) -> isize {
    inp.into_iter()
        .map(|group| group.into_iter().sum())
        .max()
        .unwrap()
}

pub fn part2(inp: &Input) -> isize {
    inp.into_iter()
        .map(|group| group.into_iter().sum::<isize>())
        .sorted()
        .rev()
        .take(3)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const EXAMPLE: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(EXAMPLE)), 24000)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(EXAMPLE)), 45000)
    }
}
