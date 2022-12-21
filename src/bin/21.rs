use std::collections::{HashMap, VecDeque};

use lazy_static::lazy_static;
use regex::Regex;

pub fn part_one(input: &str) -> Option<i64> {
    let mut instructs = parse_input(input);
    let mut map = HashMap::new();
    while let Some((name, instruct)) = instructs.pop_front() {
        match instruct {
            Instruct::Num(num) => {
                map.insert(name, num);
            }
            Instruct::Operation(op, op1, op2) => {
                if let (Some(num1), Some(num2)) = (map.get(&op1), map.get(&op2)) {
                    match op {
                        Op::Add => map.insert(name, num1 + num2),
                        Op::Sub => map.insert(name, num1 - num2),
                        Op::Mul => map.insert(name, num1 * num2),
                        Op::Div => map.insert(name, num1 / num2),
                        _ => None,
                    };
                } else {
                    instructs.push_back((name, Instruct::Operation(op, op1, op2)));
                }
            }
        }
    }
    map.get("root").copied()
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut instructs = parse_input(input);
    let mut map = HashMap::new();
    for (name, instruct) in instructs.iter_mut() {
        if name == "root" {
            if let Instruct::Operation(_, op1, op2) = instruct {
                *instruct = Instruct::Operation(Op::Equ, op1.clone(), op2.clone());
            }
        }
    }
    while let Some((name, instruct)) = instructs.pop_front() {
        if name == "humn" {
            continue;
        }
        if name == "root" {
            if let Instruct::Operation(_, op1, op2) = instruct.clone() {
                if map.get(&op1).is_some() {
                    map.insert(op2, map.get(&op1).copied().unwrap());
                    break;
                } else if map.get(&op2).is_some() {
                    map.insert(op1, map.get(&op2).copied().unwrap());
                    break;
                }
            }
        }
        match instruct {
            Instruct::Num(num) => {
                map.insert(name, num);
            }
            Instruct::Operation(op, op1, op2) => {
                if let (Some(num1), Some(num2)) = (map.get(&op1), map.get(&op2)) {
                    match op {
                        Op::Add => map.insert(name, num1 + num2),
                        Op::Sub => map.insert(name, num1 - num2),
                        Op::Mul => map.insert(name, num1 * num2),
                        Op::Div => map.insert(name, num1 / num2),
                        _ => None,
                    };
                } else {
                    instructs.push_back((name, Instruct::Operation(op, op1, op2)));
                }
            }
        }
    }
    while let Some((name, instruct)) = instructs.pop_front() {
        if !map.contains_key(&name) {
            instructs.push_back((name, instruct));
            continue;
        }
        let res = map.get(&name).copied().unwrap();
        if let Instruct::Operation(op, op1, op2) = instruct {
            if let Some(num) = map.get(&op1) {
                match op {
                    Op::Add => map.insert(op2, res - num),
                    Op::Sub => map.insert(op2, num - res),
                    Op::Mul => map.insert(op2, res / num),
                    Op::Div => map.insert(op2, num / res),
                    _ => None,
                };
            } else if let Some(num) = map.get(&op2) {
                match op {
                    Op::Add => map.insert(op1, res - num),
                    Op::Sub => map.insert(op1, num + res),
                    Op::Mul => map.insert(op1, res / num),
                    Op::Div => map.insert(op1, num * res),
                    _ => None,
                };
            } else {
                instructs.push_back((name, Instruct::Operation(op, op1, op2)));
            }
        }
    }
    Some(map.get("humn").copied().unwrap())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 21);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[derive(Debug, Clone)]
enum Instruct {
    Num(i64),
    Operation(Op, String, String),
}

#[derive(Debug, Clone)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Equ,
}

lazy_static! {
    static ref NUM_RE: Regex = Regex::new(r"(\w+): (\d+)").unwrap();
    static ref OP_RE: Regex = Regex::new(r"(\w+): (\w+) ([+-/*]) (\w+)").unwrap();
}

fn parse_input(input: &str) -> VecDeque<(String, Instruct)> {
    input
        .lines()
        .map(|s| {
            if let Some(caps) = NUM_RE.captures(s) {
                let name = caps.get(1).unwrap().as_str().to_string();
                let num = caps.get(2).unwrap().as_str().parse::<i64>().unwrap();
                (name, Instruct::Num(num))
            } else if let Some(caps) = OP_RE.captures(s) {
                let name = caps.get(1).unwrap().as_str().to_string();
                let op1 = caps.get(2).unwrap().as_str().to_string();
                let op = caps.get(3).unwrap().as_str().to_string();
                let op2 = caps.get(4).unwrap().as_str().to_string();
                match op.as_str() {
                    "+" => (name, Instruct::Operation(Op::Add, op1, op2)),
                    "-" => (name, Instruct::Operation(Op::Sub, op1, op2)),
                    "*" => (name, Instruct::Operation(Op::Mul, op1, op2)),
                    "/" => (name, Instruct::Operation(Op::Div, op1, op2)),
                    _ => panic!("Unknown op: {}", op),
                }
            } else {
                panic!("Unknown line: {}", s)
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_one(&input), Some(152));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_two(&input), Some(301));
    }
}
