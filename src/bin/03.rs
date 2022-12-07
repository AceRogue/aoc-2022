use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| find_duplicate_priority(line))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut ans = 0;
    let mut set = HashSet::new();
    for (i, line) in input.lines().enumerate() {
        if i % 3 == 0 {
            if !set.is_empty() {
                ans += set.iter().map(|c| calc_priority(*c)).sum::<u32>();
            }
            set = line.chars().collect::<HashSet<char>>();
        } else {
            let second = line.chars().collect::<HashSet<char>>();
            set.retain(|c| second.contains(c));
        }
    }
    ans += set.iter().map(|c| calc_priority(*c)).sum::<u32>();
    Some(ans)
}

fn find_duplicate_priority(s: &str) -> u32 {
    let n = s.len();
    let mut set = std::collections::HashSet::new();
    for (i, c) in s.chars().enumerate() {
        if i < n / 2 {
            set.insert(c);
        } else {
            if set.contains(&c) {
                return calc_priority(c);
            }
        }
    }
    0
}

fn calc_priority(c: char) -> u32 {
    if c.is_lowercase() {
        c as u32 - 'a' as u32 + 1
    } else if c.is_uppercase() {
        c as u32 - 'A' as u32 + 1 + 26
    } else {
        0
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
