use std::collections::{HashMap, HashSet};

use crate::*;

type Input = HashSet<(isize, isize)>;

pub fn parse_input(input: &str) -> Input {
    let mut map = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    map.insert((x as isize, y as isize));
                }
                _ => {}
            }
        }
    }
    map
}

#[derive(Debug)]
enum Dir {
    N,
    S,
    W,
    E,
}
use Dir::*;

pub fn part1(inp: &Input) -> usize {
    let mut elves = inp.clone();
    let mut dirs = [N, S, W, E];
    for _ in 0..10 {
        let mut proposed: HashMap<(isize, isize), Vec<(isize, isize)>> = HashMap::new();
        for elf in elves.iter() {
            let (x, y) = *elf;
            if ![
                (-1, -1),
                (0, -1),
                (1, -1),
                (-1, 0),
                (1, 0),
                (-1, 1),
                (0, 1),
                (1, 1),
            ]
            .iter()
            .any(|(dx, dy)| elves.contains(&(x + dx, y + dy)))
            {
                continue;
            }
            let mut propose = None;
            for dir in dirs.iter() {
                match dir {
                    N if ![(-1, -1), (0, -1), (1, -1)]
                        .iter()
                        .any(|(dx, dy)| elves.contains(&(x + dx, y + dy))) =>
                    {
                        propose = Some((x, y - 1));
                        break;
                    }
                    S if ![(-1, 1), (0, 1), (1, 1)]
                        .iter()
                        .any(|(dx, dy)| elves.contains(&(x + dx, y + dy))) =>
                    {
                        propose = Some((x, y + 1));
                        break;
                    }
                    W if ![(-1, -1), (-1, 0), (-1, 1)]
                        .iter()
                        .any(|(dx, dy)| elves.contains(&(x + dx, y + dy))) =>
                    {
                        propose = Some((x - 1, y));
                        break;
                    }
                    E if ![(1, -1), (1, 0), (1, 1)]
                        .iter()
                        .any(|(dx, dy)| elves.contains(&(x + dx, y + dy))) =>
                    {
                        propose = Some((x + 1, y));
                        break;
                    }
                    _ => {}
                }
            }
            if let Some(p) = propose {
                proposed.entry(p).or_insert(vec![]).push((x, y));
            }
        }
        for (new, proposers) in proposed.iter() {
            if proposers.len() == 1 {
                elves.remove(&proposers[0]);
                elves.insert(*new);
            }
        }
        dirs.rotate_left(1);
    }
    let max_x = *elves.iter().map(|(x, _)| x).max().unwrap();
    let max_y = *elves.iter().map(|(_, y)| y).max().unwrap();
    let min_x = *elves.iter().map(|(x, _)| x).min().unwrap();
    let min_y = *elves.iter().map(|(_, y)| y).min().unwrap();
    ((max_x - min_x + 1) * (max_y - min_y + 1)) as usize - elves.len()
}

pub fn part2(inp: &Input) -> usize {
    let mut elves = inp.clone();
    let mut dirs = [N, S, W, E];
    let mut round = 0;
    loop {
        let mut moved = false;
        let mut proposed: HashMap<(isize, isize), Vec<(isize, isize)>> = HashMap::new();
        for elf in elves.iter() {
            let (x, y) = *elf;
            if ![
                (-1, -1),
                (0, -1),
                (1, -1),
                (-1, 0),
                (1, 0),
                (-1, 1),
                (0, 1),
                (1, 1),
            ]
            .iter()
            .any(|(dx, dy)| elves.contains(&(x + dx, y + dy)))
            {
                continue;
            }
            let mut propose = None;
            for dir in dirs.iter() {
                match dir {
                    N if ![(-1, -1), (0, -1), (1, -1)]
                        .iter()
                        .any(|(dx, dy)| elves.contains(&(x + dx, y + dy))) =>
                    {
                        propose = Some((x, y - 1));
                        break;
                    }
                    S if ![(-1, 1), (0, 1), (1, 1)]
                        .iter()
                        .any(|(dx, dy)| elves.contains(&(x + dx, y + dy))) =>
                    {
                        propose = Some((x, y + 1));
                        break;
                    }
                    W if ![(-1, -1), (-1, 0), (-1, 1)]
                        .iter()
                        .any(|(dx, dy)| elves.contains(&(x + dx, y + dy))) =>
                    {
                        propose = Some((x - 1, y));
                        break;
                    }
                    E if ![(1, -1), (1, 0), (1, 1)]
                        .iter()
                        .any(|(dx, dy)| elves.contains(&(x + dx, y + dy))) =>
                    {
                        propose = Some((x + 1, y));
                        break;
                    }
                    _ => {}
                }
            }
            if let Some(p) = propose {
                proposed.entry(p).or_insert(vec![]).push((x, y));
            }
        }
        for (new, proposers) in proposed.iter() {
            if proposers.len() == 1 {
                elves.remove(&proposers[0]);
                elves.insert(*new);
                moved = true;
            }
        }
        dirs.rotate_left(1);
        round += 1;
        if !moved {
            break;
        }
    }
    round
}

fn _print_map(elves: &HashSet<(isize, isize)>) {
    let mut out = String::new();
    let max_x = *elves.iter().map(|(x, _)| x).max().unwrap();
    let max_y = *elves.iter().map(|(_, y)| y).max().unwrap();
    let min_x = *elves.iter().map(|(x, _)| x).min().unwrap();
    let min_y = *elves.iter().map(|(_, y)| y).min().unwrap();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if elves.contains(&(x, y)) {
                out.push('#')
            } else {
                out.push('.')
            }
        }
        out.push('\n')
    }
    println!("{}", out);
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const EXAMPLE: &str = "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(EXAMPLE)), 110)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(EXAMPLE)), 20)
    }
}
