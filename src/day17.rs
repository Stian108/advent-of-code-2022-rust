use std::collections::HashSet;

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
    let mut board: HashSet<(usize, usize)> = HashSet::new();
    let mut max = 0;
    for _ in 0..2022 {
        let piece = pieces.next().unwrap();
        let mut pos = (2, max + 3);
        loop {
            let mov = moves.next().unwrap();
            let new_pos = match mov {
                Move::Left if pos.0 as isize - 1 >= 0 => (pos.0 - 1, pos.1),
                Move::Right => (pos.0 + 1, pos.1),
                _ => pos,
            };
            if piece.iter().all(|cord| {
                let new_cord = (cord.0 + new_pos.0, cord.1 + new_pos.1);
                new_cord.0 < 7 && !board.contains(&new_cord)
            }) {
                pos = new_pos
            }
            if pos.1 == 0 {
                break;
            }
            let new_pos = (pos.0, pos.1 - 1);
            if piece.iter().any(|cord| {
                let new_cord = (cord.0 + new_pos.0, cord.1 + new_pos.1);
                board.contains(&new_cord)
            }) {
                break;
            }
            pos = new_pos;
        }
        for &(x, y) in piece.iter() {
            board.insert((x + pos.0, y + pos.1));
            if y + pos.1 + 1 > max {
                max = y + pos.1 + 1
            }
        }
    }
    max
}

pub fn part2(_inp: &Input) -> usize {
    0
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

    // #[test]
    // fn part2_example() {
    //     assert_eq!(part2(&parse_input(EXAMPLE)), 0)
    // }
}
