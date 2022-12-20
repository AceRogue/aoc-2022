use std::collections::{HashSet, VecDeque};

use lazy_static::lazy_static;
use rayon::prelude::*;
use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let blueprints = parse_input(input);
    let resource = Resource::default();
    let robot = Robot {
        ore: 1,
        clay: 0,
        obsidian: 0,
        geode: 0,
    };
    let ans = blueprints
        .into_par_iter()
        .map(|blueprint| {
            let geo = max_geode(&blueprint, &resource.clone(), &robot.clone(), 24);
            geo * blueprint.id
        })
        .sum::<u32>();
    Some(ans)
}

pub fn part_two(input: &str) -> Option<u32> {
    let blueprints = parse_input(input);
    let resource = Resource::default();
    let robot = Robot {
        ore: 1,
        clay: 0,
        obsidian: 0,
        geode: 0,
    };
    let ans = blueprints
        .into_par_iter()
        .take(3)
        .map(|blueprint| max_geode(&blueprint, &resource.clone(), &robot.clone(), 32))
        .product::<u32>();
    Some(ans)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 19);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"Blueprint (?P<id>\d+): Each ore robot costs (?P<ore_ore_cost>\d+) ore. Each clay robot costs (?P<clay_ore_cost>\d+) ore. Each obsidian robot costs (?P<obs_ore_cost>\d+) ore and (?P<obs_clay_cost>\d+) clay. Each geode robot costs (?P<geo_ore_cost>\d+) ore and (?P<geo_obs_cost>\d+) obsidian.").unwrap();
}

#[derive(Debug, Clone, Default, Hash, PartialEq, Eq)]
struct Resource {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
}

impl Resource {
    fn add_robot(&mut self, robot: &Robot) {
        self.ore += robot.ore;
        self.clay += robot.clay;
        self.obsidian += robot.obsidian;
        self.geode += robot.geode;
    }

    fn min(&self, other: &Self) -> Self {
        Self {
            ore: self.ore.min(other.ore),
            clay: self.clay.min(other.clay),
            obsidian: self.obsidian.min(other.obsidian),
            geode: self.geode.min(other.geode),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Robot {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
}

impl Robot {
    fn min(&self, other: &Self) -> Self {
        Self {
            ore: self.ore.min(other.ore),
            clay: self.clay.min(other.clay),
            obsidian: self.obsidian.min(other.obsidian),
            geode: self.geode.min(other.geode),
        }
    }
}

#[derive(Debug)]
struct Blueprint {
    id: u32,
    ore_ore_cost: u32,
    clay_ore_cost: u32,
    obsidian_ore_cost: u32,
    obsidian_clay_cost: u32,
    geode_ore_cost: u32,
    geode_obsidian_cost: u32,
}

impl From<&str> for Blueprint {
    fn from(s: &str) -> Self {
        let caps = RE.captures(s).unwrap();
        Self {
            id: caps["id"].parse().unwrap(),
            ore_ore_cost: caps["ore_ore_cost"].parse().unwrap(),
            clay_ore_cost: caps["clay_ore_cost"].parse().unwrap(),
            obsidian_ore_cost: caps["obs_ore_cost"].parse().unwrap(),
            obsidian_clay_cost: caps["obs_clay_cost"].parse().unwrap(),
            geode_ore_cost: caps["geo_ore_cost"].parse().unwrap(),
            geode_obsidian_cost: caps["geo_obs_cost"].parse().unwrap(),
        }
    }
}

fn parse_input(input: &str) -> Vec<Blueprint> {
    input.lines().map(Blueprint::from).collect()
}

fn max_geode(blueprint: &Blueprint, resource: &Resource, robot: &Robot, limit: u32) -> u32 {
    let mut queue = VecDeque::new();
    queue.push_back((resource.clone(), robot.clone(), 0));
    let mut max_geode = 0;
    let mut seen = HashSet::new();
    let max_ore_cost = blueprint
        .ore_ore_cost
        .max(blueprint.clay_ore_cost)
        .max(blueprint.obsidian_ore_cost)
        .max(blueprint.geode_ore_cost);

    while let Some((resource, robot, time)) = queue.pop_front() {
        if time == limit {
            max_geode = max_geode.max(resource.geode);
            continue;
        }

        let max_resource = Resource {
            ore: (limit - time) * max_ore_cost,
            clay: (limit - time) * blueprint.obsidian_clay_cost,
            obsidian: (limit - time) * blueprint.geode_obsidian_cost,
            geode: resource.geode,
        };

        let max_robot = Robot {
            ore: max_ore_cost,
            clay: blueprint.obsidian_clay_cost,
            obsidian: blueprint.geode_obsidian_cost,
            geode: robot.geode,
        };
        let tr = resource.min(&max_resource);
        let tb = robot.min(&max_robot);
        if seen.contains(&(tr.clone(), tb.clone(), time)) {
            continue;
        }
        seen.insert((tr, tb, time));

        // if can make geode robot, make it
        if resource.ore >= blueprint.geode_ore_cost
            && resource.obsidian >= blueprint.geode_obsidian_cost
        {
            let mut new_resource = resource.clone();
            let mut new_robot = robot.clone();
            new_resource.ore -= blueprint.geode_ore_cost;
            new_resource.obsidian -= blueprint.geode_obsidian_cost;
            new_resource.add_robot(&robot);
            new_robot.geode += 1;
            queue.push_back((new_resource, new_robot, time + 1));
            continue;
        }

        if resource.ore >= blueprint.ore_ore_cost && robot.ore < max_ore_cost {
            let mut new_resource = resource.clone();
            let mut new_robot = robot.clone();
            new_resource.ore -= blueprint.ore_ore_cost;
            new_resource.add_robot(&robot);
            new_robot.ore += 1;
            queue.push_back((new_resource, new_robot, time + 1));
        }
        if resource.ore >= blueprint.clay_ore_cost && robot.clay < blueprint.obsidian_clay_cost {
            let mut new_resource = resource.clone();
            let mut new_robot = robot.clone();
            new_resource.ore -= blueprint.clay_ore_cost;
            new_resource.add_robot(&robot);
            new_robot.clay += 1;
            queue.push_back((new_resource, new_robot, time + 1));
        }
        if resource.ore >= blueprint.obsidian_ore_cost
            && resource.clay >= blueprint.obsidian_clay_cost
            && robot.obsidian < blueprint.geode_obsidian_cost
        {
            let mut new_resource = resource.clone();
            let mut new_robot = robot.clone();
            new_resource.ore -= blueprint.obsidian_ore_cost;
            new_resource.clay -= blueprint.obsidian_clay_cost;
            new_resource.add_robot(&robot);
            new_robot.obsidian += 1;
            queue.push_back((new_resource, new_robot, time + 1));
        }

        let mut new_resource = resource.clone();
        new_resource.add_robot(&robot);
        queue.push_back((new_resource, robot.clone(), time + 1));
    }
    max_geode
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_one(&input), Some(33));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_two(&input), Some(3472));
    }
}
