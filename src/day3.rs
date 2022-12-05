use itertools::Itertools;
use std::collections::HashSet;
type Input = Vec<(HashSet<char>, HashSet<char>)>;

pub fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            (
                line[..line.len() / 2].chars().collect(),
                line[line.len() / 2..].chars().collect(),
            )
        })
        .collect()
}

fn priority(c: &char) -> isize {
    if c.is_lowercase() {
        *c as isize - 'a' as isize + 1
    } else if c.is_uppercase() {
        *c as isize - 'A' as isize + 27
    } else {
        panic!()
    }
}

pub fn part1(inp: &Input) -> isize {
    inp.iter()
        .map(|(left, right)| priority(left.intersection(right).next().unwrap()))
        .sum()
}

pub fn part2(inp: &Input) -> isize {
    inp.iter()
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            priority(
                chunk
                    .map(|(left, right)| left.union(&right).collect::<HashSet<_>>())
                    .reduce(|acc, set| acc.intersection(&set).cloned().collect::<HashSet<_>>())
                    .unwrap()
                    .iter()
                    .next()
                    .unwrap(),
            )
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const EXAMPLE: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(EXAMPLE)), 157)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(EXAMPLE)), 70)
    }
}
