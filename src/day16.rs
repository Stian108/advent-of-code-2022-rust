use std::collections::{HashMap, HashSet, VecDeque};

use parse_display::FromStr;

use crate::*;

type Input = (Vec<(usize, Vec<usize>)>, Vec<Vec<usize>>, Vec<usize>);

#[derive(Debug, FromStr)]
#[from_str(
    regex = "Valve (?P<id>[A-Z][A-Z]) has flow rate=(?P<flow>[0-9]+); tunnels? leads? to valves? (?P<neighbours>([A-Z][A-Z], )*[A-Z][A-Z])"
)]
pub struct Node {
    id: String,
    flow: usize,
    neighbours: VecP<String, ",">,
}

pub fn parse_input(input: &str) -> Input {
    let nodes = input.parse::<VecP<Node>>().unwrap();
    let mut node_id: HashMap<String, usize> = HashMap::new();
    let mut graph = vec![];
    for Node {
        id,
        flow,
        neighbours: _,
    } in nodes.0.iter()
    {
        node_id.insert(id.clone(), graph.len());
        graph.push((*flow, vec![]));
    }
    for Node {
        id,
        flow: _,
        neighbours,
    } in nodes.0.into_iter()
    {
        graph[*node_id.get(&id).unwrap()].1 = neighbours
            .0
            .into_iter()
            .map(|id| *node_id.get(&id).unwrap())
            .collect();
    }
    let mut has_flow: Vec<_> = graph
        .iter()
        .enumerate()
        .filter_map(|(k, (f, _))| if f > &0 { Some(k) } else { None })
        .collect();
    has_flow.insert(0, node_id["AA"]);
    let mut dist: Vec<Vec<usize>> = vec![];
    for &label in has_flow.iter() {
        let mut q: VecDeque<usize> = VecDeque::from([label]);
        let mut vis: Vec<Option<usize>> = vec![None; graph.len()];
        vis[label] = Some(0);
        while q.len() > 0 {
            let v = q.pop_front().unwrap();
            for &neighbour in graph[v].1.iter() {
                if vis[neighbour].is_none() {
                    vis[neighbour] = Some(vis[v].unwrap() + 1);
                    q.push_back(neighbour);
                }
            }
        }
        dist.push(
            has_flow
                .iter()
                .map(|&id| vis[id])
                .collect::<Option<_>>()
                .unwrap(),
        )
    }
    (graph, dist, has_flow)
}

pub fn part1(inp: &Input) -> usize {
    let (graph, dist, has_flow) = inp;
    let mut max = 0;
    let mut path = vec![0];
    find_path(&dist, graph, &has_flow, &mut path, &mut max, 0, 30, 0);
    max
}

fn find_path(
    dist: &[Vec<usize>],
    graph: &[(usize, Vec<usize>)],
    has_flow: &[usize],
    path: &mut Vec<usize>,
    max: &mut usize,
    preassure_per_time: usize,
    time_left: usize,
    current_preassure: usize,
) {
    let wait_preassure = current_preassure + preassure_per_time * time_left;
    if wait_preassure > *max {
        *max = wait_preassure;
    }
    for (ix, id) in has_flow.iter().enumerate() {
        if !path.contains(&ix) {
            let last = *path.last().unwrap();
            let preassure = graph[*id].0;
            let steps = dist[last][ix] + 1;
            if time_left as isize - steps as isize >= 0 {
                path.push(ix);
                find_path(
                    dist,
                    graph,
                    has_flow,
                    path,
                    max,
                    preassure_per_time + preassure,
                    time_left - steps,
                    current_preassure + preassure_per_time * steps,
                );
                path.pop();
            }
        }
    }
}

pub fn part2(inp: &Input) -> usize {
    let (graph, dist, has_flow) = inp;
    let mut max = 0;
    let mut visited = HashSet::from([0]);
    find_path2(
        &dist,
        graph,
        &has_flow,
        &mut visited,
        &mut max,
        26,
        0,
        0,
        0,
        0,
        0,
        0,
    );
    max
}

fn find_path2(
    dist: &[Vec<usize>],
    graph: &[(usize, Vec<usize>)],
    has_flow: &[usize],
    visited: &mut HashSet<usize>,
    max: &mut usize,
    time_left: usize,
    preassure_per_time: usize,
    current_preassure: usize,
    pos: usize,
    pos_other: usize,
    left_of_move: usize,
    preassure_to_add: usize,
) {
    let mut wait_preassure = current_preassure + preassure_per_time * time_left;
    if left_of_move < time_left {
        wait_preassure += preassure_to_add * (time_left - left_of_move)
    }
    if wait_preassure > *max {
        *max = wait_preassure;
    }
    for (ix, id) in has_flow.iter().enumerate() {
        if !visited.contains(&ix) {
            let preassure = graph[*id].0;
            let steps = dist[pos][ix] + 1;
            if steps < left_of_move {
                if time_left as isize - steps as isize >= 0 {
                    visited.insert(ix);
                    find_path2(
                        dist,
                        graph,
                        has_flow,
                        visited,
                        max,
                        time_left - steps,
                        preassure_per_time + preassure,
                        current_preassure + preassure_per_time * steps,
                        ix,
                        pos_other,
                        left_of_move - steps,
                        preassure_to_add,
                    );
                    visited.remove(&ix);
                }
            } else if steps > left_of_move {
                if time_left as isize - left_of_move as isize >= 0 {
                    visited.insert(ix);
                    find_path2(
                        dist,
                        graph,
                        has_flow,
                        visited,
                        max,
                        time_left - left_of_move,
                        preassure_per_time + preassure_to_add,
                        current_preassure + preassure_per_time * left_of_move,
                        pos_other,
                        ix,
                        steps - left_of_move,
                        preassure,
                    );
                    visited.remove(&ix);
                }
            } else {
                if time_left as isize - steps as isize >= 0 {
                    visited.insert(ix);
                    find_path2(
                        dist,
                        graph,
                        has_flow,
                        visited,
                        max,
                        time_left - steps,
                        preassure_per_time + preassure_to_add + preassure,
                        current_preassure + preassure_per_time * steps,
                        ix,
                        pos_other,
                        0,
                        0,
                    );
                    visited.remove(&ix);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const EXAMPLE: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(EXAMPLE)), 1651)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(EXAMPLE)), 1707)
    }
}
