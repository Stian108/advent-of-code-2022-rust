use crate::*;

use parse_display::FromStr;

type Input = Vec<Assignment>;

#[derive(FromStr)]
#[display("{start_left}-{end_left},{start_right}-{end_right}")]
pub struct Assignment {
    start_left: isize,
    end_left: isize,
    start_right: isize,
    end_right: isize,
}

pub fn parse_input(input: &str) -> Input {
    parse_lines(input)
}

pub fn part1(inp: &Input) -> usize {
    inp.iter()
        .filter(|pair| {
            (pair.start_left >= pair.start_right && pair.end_left <= pair.end_right)
                || (pair.start_left <= pair.start_right && pair.end_left >= pair.end_right)
        })
        .count()
}

pub fn part2(inp: &Input) -> usize {
    inp.iter()
        .filter(|pair| (pair.end_left >= pair.start_right) && (pair.start_left <= pair.end_right))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const EXAMPLE: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(EXAMPLE)), 2)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(EXAMPLE)), 4)
    }
}
