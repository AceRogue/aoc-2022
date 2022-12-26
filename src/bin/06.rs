use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    Some(start_of_packet(input, 4))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(start_of_packet(input, 14))
}

fn start_of_packet(input: &str, distinct: usize) -> u32 {
    let mut map = HashMap::new();
    let bytes = input.as_bytes();
    let (mut l, mut r) = (0, 0);
    while r < bytes.len() {
        map.entry(bytes.get(r).unwrap())
            .and_modify(|e| *e += 1)
            .or_insert(1);
        if r - l > distinct - 1 {
            let c = bytes.get(l).unwrap();
            if let Some(e) = map.get_mut(c) {
                *e -= 1;
                if *e == 0 {
                    map.remove(c);
                }
            }
            l += 1;
        }

        if r - l == distinct - 1 && map.len() == distinct {
            return r as u32 + 1;
        }
        r += 1;
    }
    unreachable!()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
