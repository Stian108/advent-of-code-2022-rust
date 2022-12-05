type Input = Vec<isize>;

pub fn parse_input(input: &str) -> Input {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn part1(inp: &Input) -> isize {
    0
}

pub fn part2(inp: &Input) -> isize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const EXAMPLE: &str = "";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(EXAMPLE)), 0)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(EXAMPLE)), 0)
    }
}
