pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| line.split_whitespace().collect::<Vec<_>>())
            .map(|line| calculate_score_for_round_part1((line[0], line[1])))
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| line.split_whitespace().collect::<Vec<_>>())
            .map(|line| calculate_score_for_round_part2((line[0], line[1])))
            .sum::<u32>(),
    )
}

fn calculate_score_for_round_part1((x, y): (&str, &str)) -> u32 {
    let score = match y {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => 0,
    };
    let round_score = match (x, y) {
        ("A", "Y") | ("B", "Z") | ("C", "X") => 6,
        ("A", "Z") | ("B", "X") | ("C", "Y") => 0,
        ("A", "X") | ("B", "Y") | ("C", "Z") => 3,
        _ => 0,
    };
    round_score + score
}

fn calculate_score_for_round_part2((x, y): (&str, &str)) -> u32 {
    let round_score = match y {
        "X" => 0,
        "Y" => 3,
        "Z" => 6,
        _ => 0,
    };
    let score = match (x, y) {
        ("A", "Y") | ("B", "X") | ("C", "Z") => 1,
        ("A", "Z") | ("B", "Y") | ("C", "X") => 2,
        ("A", "X") | ("B", "Z") | ("C", "Y") => 3,
        _ => 0,
    };
    round_score + score
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
