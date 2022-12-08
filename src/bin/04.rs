use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let lines = parse_input(input);
    Some(lines.iter().filter(|line| line.is_full_overlap()).count() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = parse_input(input);
    Some(lines.iter().filter(|line| line.is_overlap()).count() as u32)
}

#[derive(Debug)]
struct Range {
    pub start: u32,
    pub end: u32,
}

#[derive(Debug)]
struct PairRange {
    pub first: Range,
    pub second: Range,
}

impl PairRange {
    fn is_full_overlap(&self) -> bool {
        (self.first.start >= self.second.start && self.first.end <= self.second.end)
            || (self.second.start >= self.first.start && self.second.end <= self.first.end)
    }

    fn is_overlap(&self) -> bool {
        !((self.first.end < self.second.start) || (self.first.start > self.second.end))
    }
}

const REGEX: &'static str = r"(?x)(?P<left1>\d+)-(?P<right1>\d+),(?P<left2>\d+)-(?P<right2>\d+)";

fn parse_input(input: &str) -> Vec<PairRange> {
    let re = Regex::new(REGEX).unwrap();


    re.captures_iter(input)
        .map(|cap| PairRange {
            first: Range {
                start: cap["left1"].parse::<u32>().unwrap(),
                end: cap["right1"].parse::<u32>().unwrap(),
            },
            second: Range {
                start: cap["left2"].parse::<u32>().unwrap(),
                end: cap["right2"].parse::<u32>().unwrap(),
            },
        })
        .collect::<Vec<_>>()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }

    #[test]
    fn test_parse() {
        let input = String::from("2-4,6-8");
        let res = parse_input(&input);
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].first.start, 2);
    }
}
