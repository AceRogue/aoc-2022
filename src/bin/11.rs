use std::{
    collections::{BinaryHeap, VecDeque},
    u64,
};

use lazy_static::lazy_static;
use regex::Regex;

pub fn part_one(input: &str) -> Option<u64> {
    let mut monkeys = parse_input(input);
    simulate(&mut monkeys, 3, 20, false);
    Some(monkey_business(&monkeys))
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut monkeys = parse_input(input);
    let mods = monkeys.iter().map(|m| m.divisible).product::<u64>();
    simulate(&mut monkeys, mods, 10_000, true);
    Some(monkey_business(&monkeys))
}

fn monkey_business(monkeys: &[Monkey]) -> u64 {
    let mut heap = monkeys
        .iter()
        .map(|m| m.inspect_time)
        .collect::<BinaryHeap<_>>();
    let max = heap.pop().unwrap();
    let second = heap.pop().unwrap();
    max * second
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

fn simulate(monkeys: &mut Vec<Monkey>, divide: u64, round: u32, mods: bool) {
    for _ in 0..round {
        for id in 0..monkeys.len() {
            while monkeys[id].has_item() {
                let (item, monkey_id) = monkeys[id].inspect(divide, mods);
                monkeys[monkey_id as usize].add_item(item);
            }
        }
    }
}

lazy_static! {
    static ref RE: Regex = Regex::new(
        r"Monkey (?P<monkey>\d+):
\s+Starting items:\s(?P<items>[\d, ]+)
\s+Operation:\s(?P<operation>[ \S]+)
\s+Test:\sdivisible by (?P<test>\d+)
\s+If true:\sthrow to monkey (?P<true>\d+)
\s+If false:\sthrow to monkey (?P<false>\d+)"
    )
    .unwrap();
}

fn parse_input(input: &str) -> Vec<Monkey> {
    input.split("\n\n").map(Monkey::from).collect()
}

#[derive(Debug)]
enum Operation {
    Square,
    Add(u64),
    Mul(u64),
}

impl From<&str> for Operation {
    fn from(s: &str) -> Self {
        lazy_static! {
            static ref OP_RE: Regex =
                Regex::new(r"new = old (?P<op>\S) (?P<num>[\d|\S]+)").unwrap();
        };
        let caps = OP_RE.captures(s).unwrap();
        match caps["op"].as_ref() {
            "*" => {
                if let Ok(num) = caps["num"].parse::<u64>() {
                    Operation::Mul(num)
                } else {
                    Operation::Square
                }
            }
            "+" => Operation::Add(caps["num"].parse::<u64>().unwrap()),
            _ => panic!("Invalid operation"),
        }
    }
}

impl Operation {
    fn calculate(&self, input: u64) -> u64 {
        match self {
            Operation::Square => input * input,
            Operation::Add(x) => input + x,
            Operation::Mul(x) => input * x,
        }
    }
}

#[derive(Debug)]
struct Monkey {
    starting_items: VecDeque<u64>,
    operation: Operation,
    divisible: u64,
    true_branch: u64,
    false_branch: u64,
    inspect_time: u64,
}

impl From<&str> for Monkey {
    fn from(s: &str) -> Self {
        let caps = RE.captures(s).unwrap();
        let starting_items = caps["items"]
            .split(", ")
            .map(|x| x.parse::<u64>().unwrap())
            .collect();
        let operation = caps["operation"].into();
        let divisible = caps["test"].parse::<u64>().unwrap();
        let true_branch = caps["true"].parse::<u64>().unwrap();
        let false_branch = caps["false"].parse::<u64>().unwrap();
        Self {
            starting_items,
            operation,
            divisible,
            true_branch,
            false_branch,
            inspect_time: 0,
        }
    }
}

impl Monkey {
    pub fn has_item(&self) -> bool {
        !self.starting_items.is_empty()
    }

    pub fn inspect(&mut self, divide: u64, mods: bool) -> (u64, u64) {
        self.inspect_time += 1;
        let item = self.starting_items.pop_front().unwrap();
        let mut worry = self.operation.calculate(item);
        if mods {
            worry %= divide;
        } else {
            worry /= divide;
        }
        if worry % self.divisible == 0 {
            (worry, self.true_branch)
        } else {
            (worry, self.false_branch)
        }
    }

    fn add_item(&mut self, item: u64) {
        self.starting_items.push_back(item);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }

    #[test]
    fn test_regex() {
        let input = advent_of_code::read_file("examples", 11);
        let parts = input.split("\n\n");
        for part in parts {
            let caps = RE.captures(part).unwrap();
            for cap in caps.iter() {
                println!("{:?}", cap);
            }
        }
    }
}
