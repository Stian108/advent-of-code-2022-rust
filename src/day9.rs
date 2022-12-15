use parse_display::FromStr;
use std::collections::HashSet;

use crate::*;

#[derive(Debug, FromStr)]
pub enum Dir {
    L,
    R,
    U,
    D,
}

#[derive(Debug, FromStr)]
#[display("{dir} {count}")]
pub struct Move {
    dir: Dir,
    count: usize,
}

type Input = VecP<Move, "\n">;

#[derive(Debug, Hash, Clone, PartialEq, Eq, Copy)]
pub struct Point {
    x: isize,
    y: isize,
}

pub fn parse_input(input: &str) -> Input {
    input.parse().unwrap()
}

pub fn part1(inp: &Input) -> usize {
    let mut visited = HashSet::new();
    let mut head = Point { x: 0, y: 0 };
    let mut tail = Point { x: 0, y: 0 };
    for Move { count, dir } in inp.0.iter() {
        for _ in 0..*count {
            head = move_head(&head, &dir);
            tail = update_tail(&tail, &head);
            visited.insert(tail.clone());
        }
    }
    visited.len()
}

fn move_head(head: &Point, dir: &Dir) -> Point {
    match dir {
        Dir::L => Point {
            x: head.x - 1,
            y: head.y,
        },
        Dir::R => Point {
            x: head.x + 1,
            y: head.y,
        },
        Dir::U => Point {
            y: head.y + 1,
            x: head.x,
        },
        Dir::D => Point {
            y: head.y - 1,
            x: head.x,
        },
    }
}

fn update_tail(tail: &Point, head: &Point) -> Point {
    if tail.x.abs_diff(head.x) <= 1 && tail.y.abs_diff(head.y) <= 1 {
        return *tail;
    }
    let mut tail = *tail;
    tail.x += (head.x - tail.x).signum();
    tail.y += (head.y - tail.y).signum();
    tail
}

pub fn part2(inp: &Input) -> usize {
    let mut visited = HashSet::new();
    let mut rope = [Point { x: 0, y: 0 }; 10];
    for Move { count, dir } in inp.0.iter() {
        for _ in 0..*count {
            rope[0] = move_head(&rope[0], &dir);
            for i in 1..rope.len() {
                rope[i] = update_tail(&rope[i], &rope[i - 1]);
            }
            visited.insert(rope[rope.len() - 1].clone());
        }
    }
    visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const EXAMPLE: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(EXAMPLE)), 13)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(EXAMPLE)), 1)
    }

    #[test]
    fn part2_example2() {
        assert_eq!(
            part2(&parse_input(
                "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"
            )),
            36
        )
    }
}
