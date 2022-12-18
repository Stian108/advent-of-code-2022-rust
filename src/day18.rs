use std::{collections::HashSet, hash::Hash};

use parse_display::FromStr;

use crate::*;

type Input = VecP<Cube>;

#[derive(Debug, FromStr, Clone, Copy, Hash, PartialEq, Eq)]
#[display("{0},{1},{2}")]
pub struct Cube(isize, isize, isize);

pub fn parse_input(input: &str) -> Input {
    input.parse().unwrap()
}

pub fn part1(inp: &Input) -> usize {
    let mut cubes: HashSet<Cube> = HashSet::new();
    let mut sides = 0;
    for &Cube(x, y, z) in inp.0.iter() {
        sides += 6;
        if cubes.contains(&Cube(x + 1, y, z)) {
            sides -= 2;
        }
        if cubes.contains(&Cube(x - 1, y, z)) {
            sides -= 2;
        }
        if cubes.contains(&Cube(x, y + 1, z)) {
            sides -= 2;
        }
        if cubes.contains(&Cube(x, y - 1, z)) {
            sides -= 2;
        }
        if cubes.contains(&Cube(x, y, z + 1)) {
            sides -= 2;
        }
        if cubes.contains(&Cube(x, y, z - 1)) {
            sides -= 2;
        }
        cubes.insert(Cube(x, y, z));
    }
    sides
}

pub fn part2(inp: &Input) -> usize {
    let cubes: HashSet<Cube> = inp.0.iter().copied().collect();
    let max_x = cubes.iter().map(|cube| cube.0).max().unwrap() + 1;
    let min_x = cubes.iter().map(|cube| cube.0).min().unwrap() - 1;
    let max_y = cubes.iter().map(|cube| cube.1).max().unwrap() + 1;
    let min_y = cubes.iter().map(|cube| cube.1).min().unwrap() - 1;
    let max_z = cubes.iter().map(|cube| cube.2).max().unwrap() + 1;
    let min_z = cubes.iter().map(|cube| cube.2).min().unwrap() - 1;
    let mut sides = 0;
    let start = Cube(min_x, min_y, min_z);
    let mut stack = vec![start];
    let mut visited: HashSet<Cube> = HashSet::from([start]);
    while !stack.is_empty() {
        let u = stack.pop().unwrap();
        for (dx, dy, dz) in [
            (0, 0, 1),
            (0, 0, -1),
            (0, 1, 0),
            (0, -1, 0),
            (1, 0, 0),
            (-1, 0, 0),
        ]
        .iter()
        {
            let v = Cube(u.0 + dx, u.1 + dy, u.2 + dz);
            if cubes.contains(&v) {
                sides += 1;
            } else if v.0 >= min_x
                && v.0 <= max_x
                && v.1 >= min_y
                && v.1 <= max_y
                && v.2 >= min_z
                && v.2 <= max_z
                && !visited.contains(&v)
            {
                visited.insert(v);
                stack.push(v);
            }
        }
    }
    sides
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const EXAMPLE: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(EXAMPLE)), 64)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(EXAMPLE)), 58)
    }
}
