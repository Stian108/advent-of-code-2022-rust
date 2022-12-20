use good_lp::{constraint, default_solver, variables, Solution, SolverModel};

use parse_display::FromStr;

use crate::*;

type Input = VecP<Blueprint>;

#[derive(Debug, FromStr)]
#[display("Blueprint {id}: Each ore robot costs {ore_ore} ore. Each clay robot costs {clay_ore} ore. Each obsidian robot costs {obsidian_ore} ore and {obsidian_clay} clay. Each geode robot costs {geode_ore} ore and {geode_obsidian} obsidian.")]
pub struct Blueprint {
    id: usize,
    ore_ore: usize,
    clay_ore: usize,
    obsidian_ore: usize,
    obsidian_clay: usize,
    geode_ore: usize,
    geode_obsidian: usize,
}

fn solve(bp: &Blueprint, time: usize) -> usize {
    variables! {
        vars:
            0 <= ore[time] (integer);
            0 <= clay[time] (integer);
            0 <= geode[time] (integer);
            0 <= obsidian[time] (integer);
            0 <= ore_robot[time] (integer);
            0 <= clay_robot[time] (integer);
            0 <= obsidian_robot[time] (integer);
            0 <= geode_robot[time] (integer);
    }
    let mut solver = vars.maximise(geode[time - 1]).using(default_solver);
    solver.set_parameter("log", "0");
    let mut system = solver
        .with(constraint!(ore_robot[0] == 1))
        .with(constraint!(clay_robot[0] == 0))
        .with(constraint!(obsidian_robot[0] == 0))
        .with(constraint!(geode_robot[0] == 0))
        .with(constraint!(ore[0] == 0))
        .with(constraint!(clay[0] == 0))
        .with(constraint!(obsidian[0] == 0))
        .with(constraint!(geode[0] == 0));
    for i in 1..time {
        system.add_constraint(constraint!(
            ore_robot[i] + clay_robot[i] + obsidian_robot[i] + geode_robot[i]
                - ore_robot[i - 1]
                - clay_robot[i - 1]
                - obsidian_robot[i - 1]
                - geode_robot[i - 1]
                <= 1
        ));

        system.add_constraint(constraint!(
            ore_robot[i] + clay_robot[i] + obsidian_robot[i] + geode_robot[i]
                - ore_robot[i - 1]
                - clay_robot[i - 1]
                - obsidian_robot[i - 1]
                - geode_robot[i - 1]
                >= 0
        ));

        system.add_constraint(constraint!(ore_robot[i] - ore_robot[i - 1] <= 1));
        system.add_constraint(constraint!(clay_robot[i] - clay_robot[i - 1] <= 1));
        system.add_constraint(constraint!(obsidian_robot[i] - obsidian_robot[i - 1] <= 1));
        system.add_constraint(constraint!(geode_robot[i] - geode_robot[i - 1] <= 1));

        system.add_constraint(constraint!(ore_robot[i] - ore_robot[i - 1] >= 0));
        system.add_constraint(constraint!(clay_robot[i] - clay_robot[i - 1] >= 0));
        system.add_constraint(constraint!(obsidian_robot[i] - obsidian_robot[i - 1] >= 0));
        system.add_constraint(constraint!(geode_robot[i] - geode_robot[i - 1] >= 0));

        system.add_constraint(constraint!(
            ore[i]
                == ore[i - 1] + ore_robot[i - 1]
                    - (ore_robot[i] - ore_robot[i - 1]) * bp.ore_ore as f64
                    - (clay_robot[i] - clay_robot[i - 1]) * bp.clay_ore as f64
                    - (obsidian_robot[i] - obsidian_robot[i - 1]) * bp.obsidian_ore as f64
                    - (geode_robot[i] - geode_robot[i - 1]) * bp.geode_ore as f64
        ));
        system.add_constraint(constraint!(
            clay[i]
                == clay[i - 1] + clay_robot[i - 1]
                    - (obsidian_robot[i] - obsidian_robot[i - 1]) * bp.obsidian_clay as f64
        ));
        system.add_constraint(constraint!(
            obsidian[i]
                == obsidian[i - 1] + obsidian_robot[i - 1]
                    - (geode_robot[i] - geode_robot[i - 1]) * bp.geode_obsidian as f64
        ));
        system.add_constraint(constraint!(geode[i] == geode[i - 1] + geode_robot[i - 1]));

        system.add_constraint(constraint!(ore[i] >= ore_robot[i - 1]));
        system.add_constraint(constraint!(clay[i] >= clay_robot[i - 1]));
        system.add_constraint(constraint!(obsidian[i] >= obsidian_robot[i - 1]));
        system.add_constraint(constraint!(geode[i] >= geode_robot[i - 1]));
    }
    system.solve().unwrap().value(geode[time - 1]).round() as usize
}

pub fn parse_input(input: &str) -> Input {
    input.parse().unwrap()
}

pub fn part1(inp: &Input) -> usize {
    inp.0.iter().map(|bp| bp.id * solve(bp, 25)).sum()
}

pub fn part2(inp: &Input) -> usize {
    inp.0.iter().take(3).map(|bp| solve(bp, 33)).product()
}

// Too slow to bother
#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const EXAMPLE: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(EXAMPLE)), 33)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(EXAMPLE)), 62 * 56)
    }
}
