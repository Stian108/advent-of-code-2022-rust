use parse_display::FromStr;

use crate::*;

type Input = VecP<Move, "">;

#[derive(Debug, FromStr, Clone, Copy)]
pub enum Move {
    #[display("<")]
    Left,
    #[display(">")]
    Right,
}

pub fn parse_input(input: &str) -> Input {
    input.parse().unwrap()
}

pub fn part1(inp: &Input) -> usize {
    let pieces: [Vec<(usize, usize)>; 5] = [
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)],
        vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 0), (0, 1), (1, 0), (1, 1)],
    ];
    let mut pieces = pieces.into_iter().cycle();
    let mut moves = inp.0.iter().copied().cycle();
    let mut board: Vec<[bool; 7]> = vec![];
    let mut max = 0;
    for _ in 0..2022 {
        let piece = pieces.next().unwrap();
        let mut x = 2;
        let mut y = max + 3;
        loop {
            let mov = moves.next().unwrap();
            let new_x = match mov {
                Move::Left if x as isize - 1 >= 0 => x - 1,
                Move::Right => x + 1,
                _ => x,
            };
            if piece.iter().all(|cord| {
                new_x + cord.0 < 7 && (y + cord.1 >= max || !board[y + cord.1][new_x + cord.0])
            }) {
                x = new_x
            }
            if y == 0 {
                break;
            }
            let new_y = y - 1;
            if piece
                .iter()
                .any(|cord| new_y + cord.1 < max && board[new_y + cord.1][x + cord.0])
            {
                break;
            }
            y = new_y;
        }

        for &cord in piece.iter() {
            if y + cord.1 + 1 > max {
                max = y + cord.1 + 1
            }
            while board.len() < max {
                board.push([false; 7]);
            }
            board[y + cord.1][x + cord.0] = true;
        }
    }
    max
}

pub fn part2(inp: &Input) -> u128 {
    let pieces: [Vec<(usize, usize)>; 5] = [
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)],
        vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 0), (0, 1), (1, 0), (1, 1)],
    ];
    let mut pieces = pieces.into_iter().cycle();
    let mut moves = inp.0.iter().copied().cycle();
    let mut board: Vec<[bool; 7]> = vec![];
    let mut max = 0;
    let mut last_max = 0;
    let mut i = 0;
    let mut differences = vec![];
    let mut last_cycle = 0;
    let mut preamble_height = 0;
    // Must be large enough for the cycle to stabilze, and divisible by 5
    const PREAMBLE: usize = 500;
    loop {
        if i == PREAMBLE {
            preamble_height = max;
        }
        if i % 5 == 0 {
            if i > PREAMBLE {
                differences.push(max - last_max);
                if !differences.is_empty()
                    && differences[..differences.len() / 2] == differences[differences.len() / 2..]
                {
                    if differences.len() / 2 == 2 * last_cycle {
                        break;
                    }
                    last_cycle = differences.len() / 2;
                }
            }
            last_max = max;
        }
        let piece = pieces.next().unwrap();
        let mut x = 2;
        let mut y = max + 3;
        loop {
            let mov = moves.next().unwrap();
            let new_x = match mov {
                Move::Left if x as isize - 1 >= 0 => x - 1,
                Move::Right => x + 1,
                _ => x,
            };
            if piece.iter().all(|cord| {
                new_x + cord.0 < 7 && (y + cord.1 >= max || !board[y + cord.1][new_x + cord.0])
            }) {
                x = new_x
            }
            if y == 0 {
                break;
            }
            let new_y = y - 1;
            if piece
                .iter()
                .any(|cord| new_y + cord.1 < max && board[new_y + cord.1][x + cord.0])
            {
                break;
            }
            y = new_y;
        }

        for &cord in piece.iter() {
            if y + cord.1 + 1 > max {
                max = y + cord.1 + 1
            }
            while board.len() < max {
                board.push([false; 7]);
            }
            board[y + cord.1][x + cord.0] = true;
        }

        i += 1;
    }
    let cycle_height = differences[..last_cycle].iter().sum::<usize>() as u128;
    let iterations: u128 = (1000000000000 - PREAMBLE as u128) / 5;
    cycle_height * (iterations / last_cycle as u128)
        + preamble_height as u128
        + differences[..(iterations % last_cycle as u128) as usize]
            .iter()
            .sum::<usize>() as u128
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const EXAMPLE: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(EXAMPLE)), 3068)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(EXAMPLE)), 1514285714288)
    }
}
