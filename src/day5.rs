use crate::*;

use itertools::Itertools;
use parse_display::FromStr;

type Input = (Vec<Vec<char>>, Vec<Move>);

#[derive(FromStr)]
#[display("move {count} from {from} to {to}")]
pub struct Move {
    count: usize,
    from: usize,
    to: usize,
}

pub fn parse_input(input: &str) -> Input {
    let mut parts = input.split("\n\n");
    let mut stack_iter = parts.next().unwrap().lines().rev();
    let piles: usize = stack_iter
        .next()
        .unwrap()
        .split_whitespace()
        .filter_map(|w| w.parse().ok())
        .max()
        .unwrap();
    let mut stacks: Vec<Vec<char>> = vec![vec![]; piles];
    for line in stack_iter {
        for (i, c) in line.chars().skip(1).step_by(4).enumerate() {
            if !c.is_whitespace() {
                stacks[i].push(c);
            }
        }
    }
    (stacks, parse_lines(parts.next().unwrap()))
}

pub fn part1((stacks, moves): &Input) -> String {
    let mut stacks = stacks.clone();
    for Move { count, from, to } in moves {
        for _ in 0..*count {
            let pop = stacks[*from - 1].pop().unwrap();
            stacks[*to - 1].push(pop);
        }
    }
    stacks.iter().filter_map(|stack| stack.last()).join("")
}

pub fn part2((stacks, moves): &Input) -> String {
    let mut stacks = stacks.clone();
    for Move { count, from, to } in moves {
        let mut to_push = vec![];
        for _ in 0..*count {
            to_push.push(stacks[*from - 1].pop().unwrap())
        }
        stacks[*to - 1].extend(to_push.drain(..).rev());
    }
    stacks.iter().filter_map(|stack| stack.last()).join("")
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const EXAMPLE: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
    1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(EXAMPLE)), "CMZ")
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(EXAMPLE)), "MCD")
    }
}
