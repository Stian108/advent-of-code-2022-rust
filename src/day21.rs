use std::collections::HashMap;

use parse_display::FromStr;

use crate::*;

type Input = HashMap<String, Op>;

#[derive(Debug, FromStr, Clone)]
pub enum Op {
    #[display("{0} + {1}")]
    Plu(String, String),
    #[display("{0} - {1}")]
    Min(String, String),
    #[display("{0} * {1}")]
    Mul(String, String),
    #[display("{0} / {1}")]
    Div(String, String),
    #[display("{0}")]
    Const(isize),
}
use Op::*;

#[derive(Debug, FromStr, Clone)]
#[display("{0}: {1}")]
pub struct Equation(String, Op);

pub fn parse_input(input: &str) -> Input {
    input
        .parse::<VecP<Equation>>()
        .unwrap()
        .0
        .iter()
        .map(|eq| (eq.0.clone(), eq.1.clone()))
        .collect()
}

pub fn part1(inp: &Input) -> isize {
    compute(&"root", &inp)
}

pub fn compute(var: &str, eqs: &HashMap<String, Op>) -> isize {
    match eqs.get(var).unwrap() {
        Const(val) => *val,
        Plu(left, right) => compute(left, eqs) + compute(right, eqs),
        Min(left, right) => compute(left, eqs) - compute(right, eqs),
        Mul(left, right) => compute(left, eqs) * compute(right, eqs),
        Div(left, right) => compute(left, eqs) / compute(right, eqs),
    }
}

pub fn part2(inp: &Input) -> isize {
    let (left, right) = match inp.get("root").unwrap() {
        Plu(left, right) => (left, right),
        Min(left, right) => (left, right),
        Mul(left, right) => (left, right),
        Div(left, right) => (left, right),
        Const(_) => unreachable!(),
    };
    let (humn, not_humn) = match (has_humn(left, &inp), has_humn(right, &inp)) {
        (true, false) => (left, right),
        (false, true) => (right, left),
        _ => unreachable!(),
    };
    let mut humn_val = compute(not_humn, &inp);
    let mut current = humn;
    while current != "humn" {
        match inp.get(current).unwrap() {
            Plu(left, right) if has_humn(left, &inp) => {
                humn_val -= compute(right, &inp);
                current = left;
            }
            Min(left, right) if has_humn(left, &inp) => {
                humn_val += compute(right, &inp);
                current = left;
            }
            Mul(left, right) if has_humn(left, &inp) => {
                humn_val /= compute(right, &inp);
                current = left;
            }
            Div(left, right) if has_humn(left, &inp) => {
                humn_val *= compute(right, &inp);
                current = left;
            }
            Plu(left, right) if has_humn(right, &inp) => {
                humn_val -= compute(left, &inp);
                current = right;
            }
            Min(left, right) if has_humn(right, &inp) => {
                humn_val = compute(left, &inp) - humn_val;
                current = right;
            }
            Mul(left, right) if has_humn(right, &inp) => {
                humn_val /= compute(left, &inp);
                current = right;
            }
            Div(_, right) if has_humn(right, &inp) => {
                humn_val = compute(left, &inp) / humn_val;
                current = right;
            }
            _ => unreachable!(),
        }
    }
    humn_val
}

pub fn has_humn(var: &str, eqs: &HashMap<String, Op>) -> bool {
    match eqs.get(var).unwrap() {
        Const(_) => var == "humn",
        Plu(left, right) => has_humn(left, eqs) || has_humn(right, eqs),
        Min(left, right) => has_humn(left, eqs) || has_humn(right, eqs),
        Mul(left, right) => has_humn(left, eqs) || has_humn(right, eqs),
        Div(left, right) => has_humn(left, eqs) || has_humn(right, eqs),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const EXAMPLE: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(EXAMPLE)), 152)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(EXAMPLE)), 301)
    }
}
