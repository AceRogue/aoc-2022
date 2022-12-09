use std::collections::BinaryHeap;

pub fn part_one(input: &str) -> Option<u32> {
    let mut ans = u32::MIN;
    let mut sum = 0;
    for num in input.lines() {
        if num.is_empty() {
            ans = ans.max(sum);
            sum = 0;
            continue;
        }
        let n = num.parse::<u32>().unwrap();
        sum += n;
    }
    ans = ans.max(sum);
    Some(ans)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut heap = BinaryHeap::new();
    let mut sum = 0;
    for num in input.lines() {
        if num.is_empty() {
            heap.push(sum);
            sum = 0;
            continue;
        }
        let n = num.parse::<u32>().unwrap();
        sum += n;
    }
    heap.push(sum);

    let mut ans = 0;
    let mut count = 0;
    while let Some(v) = heap.pop() {
        if count == 3 {
            break;
        }
        count += 1;
        ans += v;
    }
    Some(ans)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
