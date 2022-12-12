use crate::*;

use parse_display::FromStr;

type Input = Vec<Monkey>;

#[derive(Clone, Debug, FromStr)]
#[display(
    "Monkey {_id}:
  Starting items: {items}
  Operation: {operation}
  Test: divisible by {divisible_by}
    If true: throw to monkey {if_true}
    If false: throw to monkey {if_false}"
)]
pub struct Monkey {
    _id: usize,
    items: VecP<usize>,
    operation: Op,
    divisible_by: usize,
    if_true: usize,
    if_false: usize,
}

#[derive(Debug, FromStr, Clone)]
pub enum Op {
    #[display("new = old + {0}")]
    Add(usize),
    #[display("new = old * old")]
    Sq,
    #[display("new = old * {0}")]
    Mul(usize),
}

pub fn parse_input(input: &str) -> Input {
    parse_split(input, "\n\n")
}

pub fn part1(inp: &Input) -> usize {
    let mut monkeys = inp.clone();
    let mut inspected: Vec<usize> = vec![0; inp.len()];
    for _ in 0..20 {
        for i in 0..inp.len() {
            let current = monkeys[i].items.0.clone();
            for item in current.iter() {
                inspected[i] += 1;
                let worry = match monkeys[i].operation {
                    Op::Add(val) => item + val,
                    Op::Sq => item * item,
                    Op::Mul(val) => item * val,
                } / 3;

                let target = if worry % monkeys[i].divisible_by == 0 {
                    monkeys[i].if_true
                } else {
                    monkeys[i].if_false
                };
                monkeys[target].items.0.push(worry);
            }
            monkeys[i].items.0 = vec![];
        }
    }
    inspected.sort();
    inspected[inspected.len() - 1] * inspected[inspected.len() - 2]
}

pub fn part2(inp: &Input) -> usize {
    let mut monkeys = inp.clone();
    let mut inspected: Vec<usize> = vec![0; inp.len()];
    let modulo: usize = monkeys.iter().map(|monkey| monkey.divisible_by).product();
    for _ in 0..10000 {
        for i in 0..inp.len() {
            let current = monkeys[i].items.0.clone();
            for item in current.iter() {
                inspected[i] += 1;
                let worry = match monkeys[i].operation {
                    Op::Add(val) => item + val,
                    Op::Sq => item * item,
                    Op::Mul(val) => item * val,
                } % modulo;

                let target = if worry % monkeys[i].divisible_by == 0 {
                    monkeys[i].if_true
                } else {
                    monkeys[i].if_false
                };
                monkeys[target].items.0.push(worry);
            }
            monkeys[i].items.0 = vec![];
        }
    }
    inspected.sort();
    inspected[inspected.len() - 1] * inspected[inspected.len() - 2]
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const EXAMPLE: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(EXAMPLE)), 10605)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(EXAMPLE)), 2713310158)
    }
}
