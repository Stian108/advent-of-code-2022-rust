use ndarray::{s, Array2, Axis};

type Input = Array2<usize>;

pub fn parse_input(input: &str) -> Input {
    let lines: Vec<&str> = input.lines().collect();
    let width = lines[0].len();
    let height = lines.len();
    let values: Vec<usize> = lines
        .iter()
        .flat_map(|line| line.chars().map(|c| c.to_string().parse().unwrap()))
        .collect();

    Array2::from_shape_vec((height, width), values).unwrap()
}

pub fn part1(inp: &Input) -> usize {
    let mut left: Array2<isize> = Array2::zeros(inp.raw_dim());
    let mut right: Array2<isize> = Array2::zeros(inp.raw_dim());
    for (i, row) in inp.axis_iter(Axis(0)).enumerate() {
        let mut max: isize = -1;
        for (j, e) in row.iter().enumerate() {
            left[[i, j]] = max;
            max = max.clone().max(*e as isize);
        }
        let mut max: isize = -1;
        for (j, e) in row.iter().enumerate().rev() {
            right[[i, j]] = max;
            max = max.clone().max(*e as isize);
        }
    }
    let mut up: Array2<isize> = Array2::zeros(inp.raw_dim());
    let mut down: Array2<isize> = Array2::zeros(inp.raw_dim());
    for (j, col) in inp.axis_iter(Axis(1)).enumerate() {
        let mut max: isize = -1;
        for (i, e) in col.iter().enumerate() {
            up[[i, j]] = max;
            max = max.clone().max(*e as isize);
        }
        let mut max: isize = -1;
        for (i, e) in col.iter().enumerate().rev() {
            down[[i, j]] = max;
            max = max.clone().max(*e as isize);
        }
    }
    let mut visible = 0;
    for i in 0..inp.nrows() {
        for j in 0..inp.ncols() {
            let e = inp[[i, j]];
            if [&up, &down, &left, &right]
                .iter()
                .any(|arr| arr[[i, j]] < e as isize)
            {
                visible += 1;
            }
        }
    }
    visible
}

pub fn part2(inp: &Input) -> usize {
    let mut left: Array2<usize> = Array2::ones(inp.raw_dim());
    let mut right: Array2<usize> = Array2::ones(inp.raw_dim());
    for (i, row) in inp.axis_iter(Axis(0)).enumerate() {
        for (j, e) in row.iter().enumerate() {
            let mut count = 0;
            for c in row.slice(s![..j]).iter().rev() {
                count += 1;
                if c >= e {
                    break;
                }
            }
            left[[i, j]] = count;
        }
        for (j, e) in row.iter().enumerate().rev() {
            let mut count = 0;
            for c in row.slice(s![j + 1..]).iter() {
                count += 1;
                if c >= e {
                    break;
                }
            }
            right[[i, j]] = count;
        }
    }
    let mut up: Array2<usize> = Array2::ones(inp.raw_dim());
    let mut down: Array2<usize> = Array2::ones(inp.raw_dim());
    for (j, col) in inp.axis_iter(Axis(1)).enumerate() {
        for (i, e) in col.iter().enumerate() {
            let mut count = 0;
            for c in col.slice(s![..i]).iter().rev() {
                count += 1;
                if c >= e {
                    break;
                }
            }
            down[[i, j]] = count;
        }
        for (i, e) in col.iter().enumerate().rev() {
            let mut count = 0;
            for c in col.slice(s![i + 1..]).iter() {
                count += 1;
                if c >= e {
                    break;
                }
            }
            up[[i, j]] = count;
        }
    }
    let mut max = 0;
    for i in 0..inp.nrows() {
        for j in 0..inp.ncols() {
            let e = [&up, &down, &left, &right]
                .iter()
                .map(|arr| arr[[i, j]])
                .product();
            max = max.clone().max(e);
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const EXAMPLE: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(EXAMPLE)), 21)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(EXAMPLE)), 8)
    }
}
