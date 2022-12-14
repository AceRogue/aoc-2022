use std::cmp::Ordering;

fn parse_input(input: &str) -> Vec<Pair> {
    input
        .split("\n\n")
        .into_iter()
        .map(|part| {
            let mut parts = part.lines();
            let left = Signal::from(parts.next().unwrap());
            let right = Signal::from(parts.next().unwrap());
            Pair { left, right }
        })
        .collect()
}

fn compare(a: &Value, b: &Value) -> Ordering {
    match (a, b) {
        (Value::Int(a), Value::Int(b)) => (*a).cmp(b),
        (Value::List(a), Value::List(b)) => {
            let max = a.len().max(b.len());
            for i in 0..max {
                match (a.get(i), b.get(i)) {
                    (Some(a), Some(b)) => {
                        let res = compare(a, b);
                        if res != Ordering::Equal {
                            return res;
                        }
                    }
                    (_, None) => return Ordering::Greater,
                    (None, _) => return Ordering::Less,
                }
            }
            Ordering::Equal
        }
        (Value::Int(_), Value::List(_)) => compare(&Value::List(vec![a.clone()]), b),
        (Value::List(_), Value::Int(_)) => compare(a, &Value::List(vec![b.clone()])),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let signals = parse_input(input);
    Some(
        signals
            .iter()
            .enumerate()
            .map(|(i, pair)| {
                if compare(&pair.left.0, &pair.right.0) != Ordering::Greater {
                    i as u32 + 1
                } else {
                    0
                }
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut signals = input
        .lines()
        .into_iter()
        .filter(|line| !line.is_empty())
        .map(Signal::from)
        .collect::<Vec<Signal>>();
    signals.extend(vec![
        Signal(Value::List(vec![Value::Int(2)])),
        Signal(Value::List(vec![Value::Int(6)])),
    ]);
    signals.sort_by(|a, b| compare(&a.0, &b.0));
    let res = signals
        .iter()
        .enumerate()
        .map(|(i, signal)| {
            if signal.0 == Value::List(vec![Value::Int(2)])
                || signal.0 == Value::List(vec![Value::Int(6)])
            {
                i as u32 + 1
            } else {
                1
            }
        })
        .product::<u32>();
    Some(res)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[derive(Debug)]
struct Pair {
    left: Signal,
    right: Signal,
}
#[derive(Debug)]
struct Signal(Value);

impl From<&str> for Signal {
    fn from(s: &str) -> Self {
        let mut nodes = Vec::new();
        let mut stack = Vec::new();
        let mut index = 0;
        let chars = s.chars().collect::<Vec<_>>();
        while index < s.len() {
            let c = chars[index];
            match c {
                '0'..='9' => {
                    let mut num = c.to_digit(10).unwrap();
                    while index + 1 < s.len() && chars[index + 1].is_alphanumeric() {
                        index += 1;
                        num = num * 10 + chars[index].to_digit(10).unwrap();
                    }
                    if let Some(Value::List(list)) = stack.last_mut() {
                        list.push(Value::Int(num as i32));
                    } else {
                        nodes.push(Value::Int(num as i32));
                    }
                }
                '[' => {
                    stack.push(Value::List(Vec::new()));
                }
                ']' => {
                    let list = stack.pop().unwrap();
                    if let Some(Value::List(outer_list)) = stack.last_mut() {
                        outer_list.push(list);
                    } else {
                        nodes.push(list);
                    }
                }
                _ => {}
            }
            index += 1;
        }
        Self(Value::List(nodes))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Value {
    Int(i32),
    List(Vec<Value>),
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), Some(140));
    }

    #[test]
    fn test_parse() {
        let input = advent_of_code::read_file("examples", 13);
        let input = parse_input(&input);
        println!("{:#?}", input);
    }
}
