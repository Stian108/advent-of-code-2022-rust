use std::collections::HashMap;

use itertools::Itertools;
use parse_display::FromStr;

use crate::*;

type Input = (HashMap<(usize, usize), Tile>, Vec<Move>);

#[derive(Debug, Clone, Copy)]
pub enum Tile {
    Wall,
    Floor,
}
use Tile::*;

#[derive(Debug, FromStr, Clone, Copy)]
pub enum Move {
    #[display("R")]
    R,
    #[display("L")]
    L,
    #[display("{0}")]
    S(usize),
}

#[derive(Debug, Clone, Copy)]
pub enum Facing {
    U,
    D,
    L,
    R,
}

pub fn parse_input(input: &str) -> Input {
    let (map_str, mov_str) = input.split("\n\n").collect_tuple().unwrap();
    let mut map = HashMap::new();

    for (y, line) in map_str.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    map.insert((x, y), Wall);
                }
                '.' => {
                    map.insert((x, y), Floor);
                }
                ' ' => {}
                _ => unreachable!(),
            }
        }
    }

    let mut moves = Vec::new();
    let mut last = 0;
    for (index, matched) in mov_str.match_indices(|c: char| c == 'R' || c == 'L') {
        if last != index {
            moves.push(mov_str[last..index].parse().unwrap());
        }
        moves.push(matched.parse().unwrap());
        last = index + matched.len();
    }
    if last < mov_str.len() {
        moves.push(mov_str[last..].parse().unwrap());
    }

    (map, moves)
}

pub fn part1(inp: &Input) -> usize {
    let (map, moves) = inp;
    let mut x = 0;
    let mut y = 0;
    let mut facing = Facing::R;
    while !map.contains_key(&(x, y)) {
        x += 1
    }
    let max_x = *map.keys().map(|(x, _)| x).max().unwrap();
    let max_y = *map.keys().map(|(_, y)| y).max().unwrap();
    for mov in moves.iter() {
        match mov {
            Move::R => {
                facing = match facing {
                    Facing::U => Facing::R,
                    Facing::D => Facing::L,
                    Facing::L => Facing::U,
                    Facing::R => Facing::D,
                }
            }
            Move::L => {
                facing = match facing {
                    Facing::U => Facing::L,
                    Facing::D => Facing::R,
                    Facing::L => Facing::D,
                    Facing::R => Facing::U,
                }
            }
            Move::S(steps) => {
                for _ in 0..*steps {
                    let (new_x, new_y) = match facing {
                        Facing::U if y > 0 && map.contains_key(&(x, y - 1)) => (x, y - 1),
                        Facing::D if map.contains_key(&(x, y + 1)) => (x, y + 1),
                        Facing::L if x > 0 && map.contains_key(&(x - 1, y)) => (x - 1, y),
                        Facing::R if map.contains_key(&(x + 1, y)) => (x + 1, y),
                        Facing::U => {
                            let mut new_y = max_y;
                            while !map.contains_key(&(x, new_y)) {
                                new_y -= 1;
                            }
                            (x, new_y)
                        }
                        Facing::D => {
                            let mut new_y = 0;
                            while !map.contains_key(&(x, new_y)) {
                                new_y += 1;
                            }
                            (x, new_y)
                        }
                        Facing::L => {
                            let mut new_x = max_x;
                            while !map.contains_key(&(new_x, y)) {
                                new_x -= 1;
                            }
                            (new_x, y)
                        }
                        Facing::R => {
                            let mut new_x = 0;
                            while !map.contains_key(&(new_x, y)) {
                                new_x += 1;
                            }
                            (new_x, y)
                        }
                    };
                    match map.get(&(new_x, new_y)).unwrap() {
                        Wall => break,
                        Floor => {
                            x = new_x;
                            y = new_y
                        }
                    }
                }
            }
        }
    }
    let row = (y + 1) * 1000;
    let column = (x + 1) * 4;
    let facing = match facing {
        Facing::R => 0,
        Facing::D => 1,
        Facing::L => 2,
        Facing::U => 3,
    };
    row + column + facing
}

pub fn part2(inp: &Input) -> usize {
    let (map, moves) = inp;
    let mut x = 0;
    let mut y = 0;
    let mut facing = Facing::R;
    while !map.contains_key(&(x, y)) {
        x += 1
    }
    for mov in moves.iter() {
        match mov {
            Move::R => {
                facing = match facing {
                    Facing::U => Facing::R,
                    Facing::D => Facing::L,
                    Facing::L => Facing::U,
                    Facing::R => Facing::D,
                }
            }
            Move::L => {
                facing = match facing {
                    Facing::U => Facing::L,
                    Facing::D => Facing::R,
                    Facing::L => Facing::D,
                    Facing::R => Facing::U,
                }
            }
            Move::S(steps) => {
                for _ in 0..*steps {
                    let (new_x, new_y, new_facing) = match facing {
                        Facing::U if y > 0 && map.contains_key(&(x, y - 1)) => {
                            (x, y - 1, Facing::U)
                        }
                        Facing::D if map.contains_key(&(x, y + 1)) => (x, y + 1, Facing::D),
                        Facing::L if x > 0 && map.contains_key(&(x - 1, y)) => {
                            (x - 1, y, Facing::L)
                        }
                        Facing::R if map.contains_key(&(x + 1, y)) => (x + 1, y, Facing::R),
                        Facing::U => {
                            if x < 50 {
                                (50, x + 50, Facing::R)
                            } else if x < 100 {
                                (0, x + 100, Facing::R)
                            } else {
                                (x - 100, 199, Facing::U)
                            }
                        }
                        Facing::D => {
                            if x < 50 {
                                (x + 100, 0, Facing::D)
                            } else if x < 100 {
                                (49, x + 100, Facing::L)
                            } else {
                                (99, x - 50, Facing::L)
                            }
                        }
                        Facing::L => {
                            if y < 50 {
                                (0, 149 - y, Facing::R)
                            } else if y < 100 {
                                (y - 50, 100, Facing::D)
                            } else if y < 150 {
                                (50, 149 - y, Facing::R)
                            } else {
                                (y - 100, 0, Facing::D)
                            }
                        }
                        Facing::R => {
                            if y < 50 {
                                (99, 149 - y, Facing::L)
                            } else if y < 100 {
                                (y + 50, 49, Facing::U)
                            } else if y < 150 {
                                (149, 149 - y, Facing::L)
                            } else {
                                (y - 100, 149, Facing::U)
                            }
                        }
                    };
                    match map.get(&(new_x, new_y)).unwrap() {
                        Wall => break,
                        Floor => {
                            x = new_x;
                            y = new_y;
                            facing = new_facing;
                        }
                    }
                }
            }
        }
    }
    let row = (y + 1) * 1000;
    let column = (x + 1) * 4;
    let facing = match facing {
        Facing::R => 0,
        Facing::D => 1,
        Facing::L => 2,
        Facing::U => 3,
    };
    row + column + facing
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const EXAMPLE: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(EXAMPLE)), 6032)
    }
}
