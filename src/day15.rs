use std::collections::HashSet;

use parse_display::FromStr;

use crate::*;

type Input = Vec<Sensor>;

#[derive(Debug, FromStr, PartialEq, Eq, Hash, Clone, Copy)]
#[display("x={0}, y={1}")]
pub struct Point(isize, isize);

#[derive(Debug, FromStr)]
#[display("Sensor at {location}: closest beacon is at {beacon}")]
pub struct Sensor {
    location: Point,
    beacon: Point,
}

pub fn parse_input(input: &str) -> Input {
    parse_lines(input)
}

pub fn part1(inp: &Input) -> isize {
    part1_base(inp, 2000000)
}

pub fn part1_base(inp: &Input, y: isize) -> isize {
    let mut ranges: Vec<(isize, isize)> = vec![];
    let mut beacons: HashSet<isize> = HashSet::new();
    for Sensor { location, beacon } in inp.iter() {
        if beacon.1 == y {
            beacons.insert(beacon.0);
        }
        let dist_to_beacon = location.0.abs_diff(beacon.0) + location.1.abs_diff(beacon.1);
        let dist_to_y = location.1.abs_diff(y);
        let left_at_y = dist_to_beacon as isize - dist_to_y as isize;
        if left_at_y >= 0 {
            ranges.push((location.0 - left_at_y, location.0 + left_at_y));
        }
    }
    ranges.sort();
    let mut merged_ranges: Vec<(isize, isize)> = vec![ranges[0]];
    for (start, stop) in ranges.iter().skip(1) {
        let (last_start, last_stop) = merged_ranges.pop().unwrap();
        if start > &last_stop {
            merged_ranges.push((last_start, last_stop));
            merged_ranges.push((*start, *stop));
        } else if stop > &last_stop {
            merged_ranges.push((last_start, *stop))
        } else {
            merged_ranges.push((last_start, last_stop))
        }
    }
    let covered: isize = merged_ranges
        .into_iter()
        .map(|(start, stop)| stop - start + 1)
        .sum();
    covered - beacons.len() as isize
}

pub fn part2(inp: &Input) -> isize {
    part2_base(inp, 4000000)
}

// MEGA slow. Over 14s...
pub fn part2_base(inp: &Input, max: isize) -> isize {
    let mut missing_beacon = Point(0, 0);
    for y in 0..=max {
        let mut ranges: Vec<(isize, isize)> = vec![];
        let mut beacons: HashSet<isize> = HashSet::new();
        for Sensor { location, beacon } in inp.iter() {
            if beacon.1 == y {
                beacons.insert(beacon.0);
            }
            let dist_to_beacon = location.0.abs_diff(beacon.0) + location.1.abs_diff(beacon.1);
            let dist_to_y = location.1.abs_diff(y);
            let left_at_y = dist_to_beacon as isize - dist_to_y as isize;
            if left_at_y >= 0 {
                let mut range_start = location.0 - left_at_y;
                let mut range_stop = location.0 + left_at_y;
                if range_start < 0 && range_stop >= 0 {
                    range_start = 0;
                }
                if range_stop > max && range_start <= max {
                    range_stop = max;
                }
                if range_start >= 0 && range_stop <= max {
                    ranges.push((range_start, range_stop));
                }
            }
        }
        ranges.sort();
        let mut merged_ranges: Vec<(isize, isize)> = vec![ranges[0]];
        for (start, stop) in ranges.iter().skip(1) {
            let (last_start, last_stop) = merged_ranges.pop().unwrap();
            if *start > last_stop + 1 {
                merged_ranges.push((last_start, last_stop));
                merged_ranges.push((*start, *stop));
            } else if stop > &last_stop {
                merged_ranges.push((last_start, *stop))
            } else {
                merged_ranges.push((last_start, last_stop))
            }
        }
        if merged_ranges.len() != 1 {
            missing_beacon.1 = y;
            missing_beacon.0 = merged_ranges[0].1 + 1;
        }
    }
    missing_beacon.0 * 4000000 + missing_beacon.1
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const EXAMPLE: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn part1_example() {
        assert_eq!(part1_base(&parse_input(EXAMPLE), 10), 26)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2_base(&parse_input(EXAMPLE), 20), 56000011)
    }
}
