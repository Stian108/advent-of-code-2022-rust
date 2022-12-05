use crate::util::*;
use parse_display::FromStr;

type Input = Vec<Round>;

#[derive(FromStr, PartialEq, Eq)]
pub enum RPS {
    #[display("A")]
    Rock,
    #[display("B")]
    Paper,
    #[display("C")]
    Scissor,
}

#[derive(FromStr, PartialEq, Eq)]
pub enum Res {
    #[display("X")]
    Loss,
    #[display("Y")]
    Draw,
    #[display("Z")]
    Win,
}
#[derive(FromStr)]
#[display("{opp} {me}")]
pub struct Round {
    opp: RPS,
    me: Res,
}

fn wins_against(mov: &RPS) -> RPS {
    match mov {
        RPS::Rock => RPS::Scissor,
        RPS::Paper => RPS::Rock,
        RPS::Scissor => RPS::Paper,
    }
}

fn looses_against(mov: &RPS) -> RPS {
    match mov {
        RPS::Scissor => RPS::Rock,
        RPS::Rock => RPS::Paper,
        RPS::Paper => RPS::Scissor,
    }
}

fn points(mov: &RPS) -> isize {
    match mov {
        RPS::Rock => 1,
        RPS::Paper => 2,
        RPS::Scissor => 3,
    }
}

fn convert(mov: &Res) -> RPS {
    match mov {
        Res::Loss => RPS::Rock,
        Res::Draw => RPS::Paper,
        Res::Win => RPS::Scissor,
    }
}

pub fn parse_input(input: &str) -> Input {
    parse_lines(input)
}

pub fn part1(inp: &Input) -> isize {
    inp.iter()
        .map(|Round { me, opp }| {
            let me = convert(me);
            if opp == &me {
                3 + points(&me)
            } else if &wins_against(&me) == opp {
                6 + points(&me)
            } else if &looses_against(&me) == opp {
                points(&me)
            } else {
                unreachable!()
            }
        })
        .sum()
}

pub fn part2(inp: &Input) -> isize {
    inp.iter()
        .map(|Round { me, opp }| match me {
            Res::Win => 6 + points(&looses_against(opp)),
            Res::Loss => points(&wins_against(opp)),
            Res::Draw => 3 + points(opp),
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const EXAMPLE: &str = "A Y
B X
C Z";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(EXAMPLE)), 15)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(EXAMPLE)), 12)
    }
}
