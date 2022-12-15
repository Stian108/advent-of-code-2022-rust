use itertools::Itertools;
use parse_display::FromStr;

use crate::*;

type Input = Vec<isize>;

#[derive(Debug, FromStr)]
pub enum Op {
    #[display("noop")]
    Noop,
    #[display("addx {0}")]
    Addx(isize),
}

pub fn parse_input(input: &str) -> Input {
    let mut xs: Vec<isize> = vec![1];
    let mut x: isize = 1;
    for op in input.parse::<VecP<Op, "\n">>().unwrap().0 {
        match op {
            Op::Noop => xs.push(x),
            Op::Addx(val) => {
                xs.push(x);
                xs.push(x);
                x += val;
            }
        }
    }
    xs
}

pub fn part1(inp: &Input) -> isize {
    inp.iter()
        .enumerate()
        .skip(20)
        .step_by(40)
        .take(6)
        .map(|(i, &e)| i as isize * e)
        .sum()
}

pub fn part2(inp: &Input) -> String {
    let mut pixels: String = String::new();
    for chunk in inp.iter().skip(1).chunks(40).into_iter() {
        for (i, &x) in chunk.enumerate() {
            let i = i as isize;
            if i == x || i == x + 1 || i == x - 1 {
                pixels.push('#');
            } else {
                pixels.push('.')
            }
        }
        pixels.push('\n');
    }
    pixels
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const EXAMPLE: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(EXAMPLE)), 13140)
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse_input(EXAMPLE)),
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"
        )
    }
}
