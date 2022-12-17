use itertools::Itertools;
use lazy_static::lazy_static;
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

use rayon::prelude::*;

pub fn part_one(input: &str) -> Option<u32> {
    let valve_map = build_valves(input);
    Some(max_pressure(&valve_map, 30))
}



pub fn part_two(input: &str) -> Option<u32> {
    let valve_map = build_valves(input);
    let nodes: BTreeSet<&str> = valve_map
        .keys()
        .filter(|name| name != &"AA")
        .map(|name| name.as_ref())
        .collect();
    let powerset_paths: Vec<BTreeSet<&str>> = nodes
        .clone()
        .into_iter()
        .powerset()
        .map(|set| set.into_iter().collect::<BTreeSet<_>>())
        .collect_vec();

    println!("{:?}", powerset_paths);

    let ans = powerset_paths
        .into_par_iter()
        .map(|path| {
            let diff = nodes.difference(&path).copied().collect::<BTreeSet<_>>();
            // println! {"path: {:?}, diff: {:?}", path, diff};
            let human = build_path_valves(&path, &valve_map);
            let elephant = build_path_valves(&diff, &valve_map);
            let human_max = max_pressure(&human, 26);
            let elephant_max = max_pressure(&elephant, 26);
            human_max + elephant_max
        })
        .max()
        .unwrap();

    // let mut cache = HashMap::new();
    // let mut ans = 0;
    // for path in powerset_paths {
    //     let diff = nodes.difference(&path).copied().collect::<BTreeSet<_>>();
    //     println!{"path: {:?}, diff: {:?}", path, diff};
    //     let res = if cache.contains_key(&path) {
    //         cache.get(&path).unwrap() + cache.get(&diff).unwrap()
    //     } else {
    //         let human = build_path_valves(&path, &valve_map);
    //         let elephant = build_path_valves(&diff, &valve_map);
    //         let human_max = max_pressure(&human, 26);
    //         let elephant_max = max_pressure(&elephant, 26);
    //         cache.insert(path, human_max);
    //         cache.insert(diff, elephant_max);
    //         human_max + elephant_max
    //     };
    //     ans = ans.max(res);
    // }

    Some(ans)
}

fn build_path_valves(
    path: &BTreeSet<&str>,
    valves: &HashMap<String, Valve>,
) -> HashMap<String, Valve> {
    let mut res = path
        .iter()
        .map(|&name| (name.to_string(), valves[name].clone()))
        .collect::<HashMap<_, _>>();
    res.insert("AA".to_string(), valves["AA"].clone());
    res
}

fn max_pressure(valves: &HashMap<String, Valve>, time: u32) -> u32 {
    let mut ans = 0;
    let mut queue: VecDeque<(&str, u32, u32, BTreeSet<&str>)> = VecDeque::new();
    queue.push_back(("AA", time, 0, BTreeSet::new()));

    let mut cache = HashMap::new();
    while let Some((name, time, pressure, visited)) = queue.pop_front() {
        let prev = cache
            .get(&(name, time, visited.clone()))
            .copied()
            .unwrap_or(0);
        if prev > pressure {
            continue;
        }
        cache.insert((name, time, visited.clone()), pressure);
        ans = ans.max(pressure);

        let valve = valves.get(name).unwrap();

        for (next, dist) in valve.neighbors.iter() {
            if time <= dist + 1 {
                continue;
            }
            let minute = time - dist - 1;
            if !visited.contains(&next.as_ref()) && valves.contains_key(next) {
                let mut visited = visited.clone();
                visited.insert(next);
                queue.push_back((
                    next,
                    minute,
                    pressure + valves[next].flow_rate * minute,
                    visited,
                ));
            }
        }
    }
    ans
}

fn get_neighbors(start: &str, edges: &HashMap<String, Vec<String>>) -> HashMap<String, u32> {
    let mut ans = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));
    while let Some((name, distance)) = queue.pop_front() {
        let pre = ans.get(name).copied().unwrap_or(u32::MAX);
        if distance < pre {
            ans.insert(name.to_string(), distance);

            for next in edges[name].iter() {
                queue.push_back((next, distance + 1));
            }
        }
    }
    ans.remove(start);
    ans
}

fn build_valves(input: &str) -> HashMap<String, Valve> {
    let valves = parse_input(input);
    let (mut valves_map, mut valve_edges) = (HashMap::new(), HashMap::new());
    for valve in valves {
        valve_edges.insert(valve.name.clone(), valve.tunnels.clone());
        valves_map.insert(valve.name.clone(), valve.clone());
    }
    for (name, valve) in valves_map.iter_mut() {
        valve.neighbors = get_neighbors(name, &valve_edges);
    }

    // remove useless neighbors where flow_rate is 0
    valves_map.retain(|name, valve| name == "AA" || valve.flow_rate > 0);
    let keys = valves_map.keys().cloned().collect::<HashSet<_>>();
    for valve in valves_map.values_mut() {
        valve.neighbors.retain(|name, _| keys.contains(name));
    }
    valves_map
}

lazy_static!(
    static ref RE: regex::Regex = regex::Regex::new(r"Valve (?P<source>\w+) has flow rate=(?P<rate>\d+); tunnels? leads? to valves? (?P<target>[\w+|,| ]+)").unwrap();
);

#[derive(Debug, Clone)]
struct Valve {
    name: String,
    flow_rate: u32,
    tunnels: Vec<String>,
    neighbors: HashMap<String, u32>,
}

fn parse_input(input: &str) -> Vec<Valve> {
    let mut valves = Vec::new();
    for line in input.lines() {
        let captures = RE.captures(line).unwrap();
        let name = captures["source"].to_string();
        let flow_rate = captures["rate"].parse::<u32>().unwrap();
        let mut tunnels = Vec::new();
        for tunnel in captures["target"].split(", ") {
            tunnels.push(tunnel.to_string());
        }
        valves.push(Valve {
            name: name.to_string(),
            flow_rate,
            tunnels,
            neighbors: HashMap::new(),
        });
    }

    valves
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 16);
    // advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_one(&input), Some(1651));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_two(&input), Some(1707));
    }
}
