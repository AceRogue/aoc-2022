use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, VecDeque};

pub fn part_one(input: &str) -> Option<String> {
    let (mut stack, commands) = parse_input(input);
    for command in commands {
        stack.exec_command(&command);
    }
    Some(stack.top_crates().iter().collect())
}

pub fn part_two(input: &str) -> Option<String> {
    let (mut stack, commands) = parse_input(input);
    for command in commands {
        stack.exec_command_part2(&command);
    }
    Some(stack.top_crates().iter().collect())
}

#[derive(Debug)]
struct CrateStacks(HashMap<u32, VecDeque<char>>);

impl CrateStacks {
    fn exec_command(&mut self, command: &Command) {
        let mut n = command.num;
        while n > 0 && !self.0[&command.from].is_empty() {
            let c = self.0.get_mut(&command.from).unwrap().pop_back().unwrap();
            let entry = self.0.entry(command.to).or_default();
            entry.push_back(c);
            n -= 1;
        }
    }

    fn exec_command_part2(&mut self, command: &Command) {
        let mut n = command.num;
        let mut cand = VecDeque::new();
        while n > 0 && !self.0[&command.from].is_empty() {
            let c = self.0.get_mut(&command.from).unwrap().pop_back().unwrap();
            cand.push_back(c);
            n -= 1;
        }
        let entry = self.0.entry(command.to).or_default();
        while let Some(c) = cand.pop_back() {
            entry.push_back(c);
        }
    }

    fn top_crates(&self) -> Vec<char> {
        let mut crates = Vec::new();
        for i in 1..=self.0.len() {
            let cs = self.0.get(&(i as u32)).unwrap();
            if !cs.is_empty() {
                crates.push(*cs.back().unwrap())
            }
        }
        crates
    }
}

#[derive(Debug)]
struct Command {
    num: u32,
    from: u32,
    to: u32,
}

lazy_static! {
    static ref COMMAND_REGEX: Regex =
        Regex::new(r"move (?P<num>\d+) from (?P<from>\d+) to (?P<to>\d+)").unwrap();
}

impl From<&str> for Command {
    fn from(s: &str) -> Self {
        let caps = COMMAND_REGEX.captures(s).unwrap();
        Command {
            num: caps["num"].parse::<u32>().unwrap(),
            from: caps["from"].parse::<u32>().unwrap(),
            to: caps["to"].parse::<u32>().unwrap(),
        }
    }
}

fn parse_input(input: &str) -> (CrateStacks, Vec<Command>) {
    let mut lines = input.lines();
    let mut stacks = CrateStacks(HashMap::new());
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        for (i, c) in line.chars().enumerate() {
            if !c.is_alphabetic() {
                continue;
            }
            let index = i / 4 + 1;
            let stack = stacks.0.entry(index as u32).or_insert_with(VecDeque::new);
            stack.push_front(c);
        }
    }

    // parse commands
    let mut commands: Vec<Command> = Vec::new();
    for line in lines {
        commands.push(line.into());
    }

    (stacks, commands)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }

    #[test]
    fn test_parse_input() {
        let input = advent_of_code::read_file("examples", 5);
        let (stack, commands) = parse_input(&input);
        assert!(stack.0.get(&2).is_some());
        assert_eq!(stack.0.get(&2).unwrap().len(), 3);
        assert_eq!(commands.len(), 4);
    }
}
