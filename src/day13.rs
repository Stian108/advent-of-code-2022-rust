use crate::*;

use itertools::EitherOrBoth::{Both, Left, Right};
use itertools::Itertools;
use std::cmp::Ordering;

type Input = Vec<(Tree<usize>, Tree<usize>)>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tree<T> {
    Leaf(T),
    Node(Vec<Tree<T>>),
}
use Tree::*;

impl<T: std::str::FromStr> FromStr for Tree<T>
where
    T::Err: Debug,
    T: Debug,
{
    type Err = T::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tree = parse_tree(&mut s.chars())?.pop().unwrap();
        Ok(tree)
    }
}

fn parse_tree<T: FromStr>(chars: &mut std::str::Chars) -> Result<Vec<Tree<T>>, T::Err>
where
    T::Err: Debug,
{
    let mut children = vec![];
    let mut val = String::new();
    while let Some(i) = chars.next() {
        match i {
            '[' => children.push(Node(parse_tree(chars)?)),
            ']' => {
                if !val.is_empty() {
                    children.push(Leaf(val.parse()?));
                }
                break;
            }
            ',' => {
                if !val.is_empty() {
                    children.push(Leaf(val.parse()?));
                }
                val = String::new();
            }
            d if d.is_digit(10) => val.push(d),
            _ => unreachable!(),
        }
    }
    Ok(children)
}

fn compare<T: Ord + Copy>(left: &Tree<T>, right: &Tree<T>) -> Ordering {
    match (left, right) {
        (Leaf(l), Leaf(r)) => l.cmp(r),
        (Node(_), Leaf(r)) => compare(left, &Node(vec![Leaf(*r)])),
        (Leaf(l), Node(_)) => compare(&Node(vec![Leaf(*l)]), right),
        (Node(l), Node(r)) => l
            .iter()
            .zip_longest(r.iter())
            .find_map(|e| match e {
                Both(l, r) => match compare(l, r) {
                    Ordering::Equal => None,
                    ord => Some(ord),
                },
                Right(_) => Some(Ordering::Less),
                Left(_) => Some(Ordering::Greater),
            })
            .unwrap_or(Ordering::Equal),
    }
}

pub fn parse_input(input: &str) -> Input {
    input
        .split("\n\n")
        .map(|pairs| {
            pairs
                .lines()
                .map(|tree| tree.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

pub fn part1(inp: &Input) -> usize {
    inp.iter()
        .enumerate()
        .filter_map(|(ix, (l, r))| match compare(l, r) {
            Ordering::Less => Some(ix + 1),
            _ => None,
        })
        .sum()
}

pub fn part2(inp: &Input) -> usize {
    let dividers: [Tree<usize>; 2] = ["[[2]]".parse().unwrap(), "[[6]]".parse().unwrap()];
    let mut packets: Vec<&Tree<usize>> = dividers.iter().collect();
    for (l, r) in inp.iter() {
        packets.push(l);
        packets.push(r)
    }
    packets.sort_by(|a, b| compare(a, b));
    packets
        .iter()
        .enumerate()
        .filter_map(|(ix, &e)| {
            if dividers.iter().any(|d| d == e) {
                Some(ix + 1)
            } else {
                None
            }
        })
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const EXAMPLE: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(EXAMPLE)), 13)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(EXAMPLE)), 140)
    }
}
