use std::collections::{HashSet, VecDeque};

use crate::*;

type Input = Vec<Vec<Option<Vec<Tile>>>>;

#[derive(Debug, Clone, Copy)]
pub enum Tile {
    Up,
    Down,
    Left,
    Right,
}
use Tile::*;

pub fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '<' => Some(vec![Left]),
                    '>' => Some(vec![Right]),
                    '^' => Some(vec![Up]),
                    'v' => Some(vec![Down]),
                    '.' => Some(vec![]),
                    '#' => None,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

fn move_tiles(inp: &Input) -> Input {
    let height = inp.len();
    let width = inp[0].len();
    let mut out = vec![vec![Some(vec![]); width]; height];
    for (y, row) in inp.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if let Some(spikes) = tile {
                for spike in spikes.iter() {
                    match spike {
                        Up => {
                            if y == 1 {
                                if let Some(tiles) = &mut out[height - 2][x] {
                                    tiles.push(Up);
                                }
                            } else {
                                if let Some(tiles) = &mut out[y - 1][x] {
                                    tiles.push(Up);
                                }
                            }
                        }
                        Down => {
                            if y == height - 2 {
                                if let Some(tiles) = &mut out[1][x] {
                                    tiles.push(Down);
                                }
                            } else {
                                if let Some(tiles) = &mut out[y + 1][x] {
                                    tiles.push(Down);
                                }
                            }
                        }
                        Left => {
                            if x == 1 {
                                if let Some(tiles) = &mut out[y][width - 2] {
                                    tiles.push(Left);
                                }
                            } else {
                                if let Some(tiles) = &mut out[y][x - 1] {
                                    tiles.push(Left);
                                }
                            }
                        }
                        Right => {
                            if x == width - 2 {
                                if let Some(tiles) = &mut out[y][1] {
                                    tiles.push(Right);
                                }
                            } else {
                                if let Some(tiles) = &mut out[y][x + 1] {
                                    tiles.push(Right);
                                }
                            }
                        }
                    }
                }
            } else {
                out[y][x] = None
            }
        }
    }
    out
}

pub fn part1(inp: &Input) -> usize {
    let goal = (inp[0].len() - 2, inp.len() - 1);
    let mut q: VecDeque<(usize, usize, usize)> = VecDeque::from([(1, 0, 0)]);
    let mut tiles: Input = move_tiles(&inp);
    let mut visited: HashSet<(usize, usize, usize)> = HashSet::from([(1, 0, 0)]);
    let mut max_time = 0;
    loop {
        let (x, y, time) = q.pop_front().unwrap();
        if time > max_time {
            max_time = time;
            tiles = move_tiles(&tiles);
        }
        if x == goal.0 && y == goal.1 {
            break time;
        }
        for &(dx, dy) in [(0, 0), (1, 0), (-1, 0), (0, 1), (0, -1)].iter() {
            if x as isize + dx >= 0 && y as isize + dy >= 0 {
                let x = (x as isize + dx) as usize;
                let y = (y as isize + dy) as usize;
                if !visited.contains(&(x, y, time + 1)) {
                    if let Some(line) = tiles.get(y) {
                        if let Some(Some(tiles)) = line.get(x) {
                            if tiles.is_empty() {
                                visited.insert((x, y, time + 1));
                                q.push_back((x, y, time + 1));
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn part2(inp: &Input) -> usize {
    let goal = (inp[0].len() - 2, inp.len() - 1);
    let start = (1, 0);
    let mut q: VecDeque<(usize, usize, usize, usize)> = VecDeque::from([(1, 0, 0, 0)]);
    let mut tiles: Input = move_tiles(&inp);
    let mut visited: HashSet<(usize, usize, usize, usize)> = HashSet::from([(1, 0, 0, 0)]);
    let mut max_time = 0;
    loop {
        let (x, y, time, trip) = q.pop_front().unwrap();
        let mut trip = trip;
        if time > max_time {
            max_time = time;
            tiles = move_tiles(&tiles);
        }
        if trip % 2 == 0 && x == goal.0 && y == goal.1 {
            if trip == 2 {
                break time;
            } else {
                trip += 1;
            }
        } else if trip % 2 == 1 && x == start.0 && y == start.1 {
            trip += 1;
        }
        for &(dx, dy) in [(0, 0), (1, 0), (-1, 0), (0, 1), (0, -1)].iter() {
            if x as isize + dx >= 0 && y as isize + dy >= 0 {
                let x = (x as isize + dx) as usize;
                let y = (y as isize + dy) as usize;
                if !visited.contains(&(x, y, time + 1, trip)) {
                    if let Some(line) = tiles.get(y) {
                        if let Some(Some(tiles)) = line.get(x) {
                            if tiles.is_empty() {
                                visited.insert((x, y, time + 1, trip));
                                q.push_back((x, y, time + 1, trip));
                            }
                        }
                    }
                }
            }
        }
    }
}

fn _print_map(inp: &Input) {
    let mut out = String::new();
    for row in inp.iter() {
        for tile in row.iter() {
            out.push(if let Some(spikes) = tile {
                if spikes.len() > 0 {
                    '*'
                } else {
                    '.'
                }
            } else {
                '#'
            })
        }
        out.push('\n')
    }
    println!("{}", out);
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const EXAMPLE: &str = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(EXAMPLE)), 18)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(EXAMPLE)), 54)
    }
}
