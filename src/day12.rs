use ndarray::Array2;
use std::collections::VecDeque;

type Input = (Array2<usize>, (usize, usize), (usize, usize));

pub fn parse_input(input: &str) -> Input {
    let lines: Vec<&str> = input.lines().collect();
    let width = lines[0].len();
    let height = lines.len();
    let values: Vec<usize> = lines
        .iter()
        .flat_map(|line| {
            line.chars().map(|c| match c {
                'S' => 'a',
                'E' => 'z',
                _ => c,
            } as usize)
        })
        .collect();
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == 'S' {
                start = (i, j);
            }
            if c == 'E' {
                end = (i, j);
            }
        }
    }
    (
        Array2::from_shape_vec((height, width), values).unwrap(),
        start,
        end,
    )
}

pub fn solve(g: &Array2<usize>, s: (usize, usize), e: (usize, usize)) -> Option<usize> {
    let mut q: VecDeque<(usize, usize)> = VecDeque::from([s]);
    let mut vis: Array2<Option<usize>> = Array2::from_elem(g.raw_dim(), None);
    vis[[s.0, s.1]] = Some(0);
    while q.len() > 0 {
        let v = q.pop_front().unwrap();
        let v_c = g[[v.0, v.1]];
        let v_dist = vis[[v.0, v.1]].unwrap();
        if v == e {
            break;
        }
        for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)].iter() {
            let w = (v.0 as isize + dx, v.1 as isize + dy);
            if w.0 >= 0 && w.0 < g.nrows() as isize && w.1 >= 0 && w.1 < g.ncols() as isize {
                let w = (w.0 as usize, w.1 as usize);
                let w_c = g[[w.0, w.1]];
                if w_c <= v_c + 1 && vis[[w.0, w.1]].is_none() {
                    vis[[w.0, w.1]] = Some(v_dist + 1);
                    q.push_back(w);
                }
            }
        }
    }
    vis[[e.0, e.1]]
}

pub fn part1(inp: &Input) -> usize {
    let (g, s, e) = inp.clone();
    solve(&g, s, e).unwrap()
}

pub fn part2(inp: &Input) -> usize {
    let (g, _, e) = inp.clone();
    let starts: Vec<(usize, usize)> = g
        .indexed_iter()
        .filter_map(|(ix, &c)| if c == 'a' as usize { Some(ix) } else { None })
        .collect();
    starts
        .iter()
        .filter_map(|&ix| solve(&g, ix, e))
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const EXAMPLE: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(EXAMPLE)), 31)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(EXAMPLE)), 29)
    }
}
