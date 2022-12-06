fn solve(inp: &str, len: usize) -> usize {
    inp.chars()
        .collect::<Vec<_>>()
        .windows(len)
        .position(|window| (1..len).all(|i| !window[i..].contains(&window[i - 1])))
        .unwrap()
        + len
}

pub fn part1(inp: &str) -> usize {
    solve(inp, 4)
}

pub fn part2(inp: &str) -> usize {
    solve(inp, 14)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&"mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&"mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19)
    }
}
