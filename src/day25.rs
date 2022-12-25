pub fn part1(inp: &str) -> String {
    to_snafu(inp.lines().map(|val| from_snafu(val)).sum())
}

// Conversion to balanced quinary, based on conversion to balanced ternary.
fn to_snafu(sum: isize) -> String {
    let mut out: String = String::new();
    let mut n = sum;
    while n > 0 {
        let mut rem = n % 5;
        n /= 5;
        if rem == 3 {
            rem = -2;
            n += 1;
        } else if rem == 4 {
            rem = -1;
            n += 1;
        }
        out.insert(
            0,
            match rem {
                0 => '0',
                1 => '1',
                2 => '2',
                -1 => '-',
                -2 => '=',
                _ => unreachable!(),
            },
        )
    }
    out
}

fn from_snafu(val: &str) -> isize {
    val.chars().fold(0, |acc, c| {
        5 * acc
            + match c {
                '0' => 0,
                '1' => 1,
                '2' => 2,
                '-' => -1,
                '=' => -2,
                _ => unreachable!(),
            }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const EXAMPLE: &str = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), "2=-1=0")
    }
}
