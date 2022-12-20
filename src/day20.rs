use itertools::Itertools;

use crate::*;

type Input = VecP<isize>;

pub fn parse_input(input: &str) -> Input {
    input.parse().unwrap()
}

pub fn part1(inp: &Input) -> isize {
    let mut list: Vec<(isize, usize, usize)> = inp.0.iter().map(|&e| (e, 0, 0)).collect();
    let zero_index = inp.0.iter().find_position(|&e| *e == 0).unwrap().0;
    for i in 0..list.len() - 1 {
        list[i].1 = i + 1;
        list[i + 1].2 = i;
    }
    list[0].2 = list.len() - 1;
    for (ix, mov) in inp.0.iter().enumerate() {
        if *mov == 0 {
            continue;
        }
        let mut mov = *mov;
        let mut target = ix;
        if mov < 0 {
            target = list[target].2;
        }
        loop {
            match mov.cmp(&0) {
                std::cmp::Ordering::Less => {
                    target = list[target].2;
                    if target == ix {
                        target = list[target].2;
                    }
                    mov += 1
                }
                std::cmp::Ordering::Greater => {
                    target = list[target].1;
                    if target == ix {
                        target = list[target].1;
                    }
                    mov -= 1
                }
                std::cmp::Ordering::Equal => break,
            }
        }
        let to = list[ix].1;
        let from = list[ix].2;
        let target_to = list[target].1;

        list[target].1 = ix;
        list[ix].2 = target;

        list[ix].1 = target_to;
        list[target_to].2 = ix;

        list[from].1 = to;
        list[to].2 = from;
    }
    let mut i = 0;
    let mut ix = zero_index;
    let mut sum = 0;
    loop {
        if i == 1000 || i == 2000 || i == 3000 {
            sum += list[ix].0;
            if i == 3000 {
                break;
            }
        }
        ix = list[ix].1;
        i += 1;
    }
    sum
}

pub fn part2(inp: &Input) -> isize {
    let key = 811589153;
    let mut list: Vec<(isize, usize, usize)> = inp.0.iter().map(|&e| (e, 0, 0)).collect();
    let zero_index = inp.0.iter().find_position(|&e| *e == 0).unwrap().0;
    for i in 0..list.len() - 1 {
        list[i].1 = i + 1;
        list[i + 1].2 = i;
    }
    list[0].2 = list.len() - 1;
    for _ in 0..10 {
        for (ix, mov) in inp.0.iter().enumerate() {
            let mut mov = (mov * key) % (list.len() as isize - 1);
            if mov == 0 {
                continue;
            }
            let mut target = ix;
            if mov < 0 {
                target = list[target].2;
            }
            loop {
                match mov.cmp(&0) {
                    std::cmp::Ordering::Less => {
                        target = list[target].2;
                        if target == ix {
                            target = list[target].2;
                        }
                        mov += 1
                    }
                    std::cmp::Ordering::Greater => {
                        target = list[target].1;
                        if target == ix {
                            target = list[target].1;
                        }
                        mov -= 1
                    }
                    std::cmp::Ordering::Equal => break,
                }
            }
            let to = list[ix].1;
            let from = list[ix].2;
            let target_to = list[target].1;

            list[target].1 = ix;
            list[ix].2 = target;

            list[ix].1 = target_to;
            list[target_to].2 = ix;

            list[from].1 = to;
            list[to].2 = from;
        }
    }
    let mut i = 0;
    let mut ix = zero_index;
    let mut sum = 0;
    loop {
        if i == 1000 || i == 2000 || i == 3000 {
            sum += list[ix].0 * key;
            if i == 3000 {
                break;
            }
        }
        ix = list[ix].1;
        i += 1;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const EXAMPLE: &str = "1
2
-3
3
-2
0
4";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(EXAMPLE)), 3)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(EXAMPLE)), 1623178306)
    }
}
