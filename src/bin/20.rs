pub fn part_one(input: &str) -> Option<i64> {
    let numbers = parse_input(input);
    let mut new_nums = numbers.clone();
    decrypt(&numbers, &mut new_nums);

    let n = numbers.len();
    let mut index = 0;
    for (i, num) in new_nums.iter().enumerate() {
        if num.1 == 0 {
            index = i;
            break;
        }
    }
    Some(
        new_nums[(index + 1000) % n].1
            + new_nums[(index + 2000) % n].1
            + new_nums[(index + 3000) % n].1,
    )
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut numbers = parse_input(input);
    let decrypted_key = 811589153;
    for num in numbers.iter_mut() {
        num.1 *= decrypted_key;
    }
    let mut new_nums = numbers.clone();
    for _ in 0..10 {
        decrypt(&numbers, &mut new_nums);
    }

    let n = numbers.len();
    let mut index = 0;
    for (i, num) in new_nums.iter().enumerate() {
        if num.1 == 0 {
            index = i;
            break;
        }
    }
    Some(
        new_nums[(index + 1000) % n].1
            + new_nums[(index + 2000) % n].1
            + new_nums[(index + 3000) % n].1,
    )
}

fn decrypt(nums: &Vec<(usize, i64)>, new_nums: &mut Vec<(usize, i64)>) {
    let n = nums.len();
    for &num in nums.iter() {
        for (i, &t) in new_nums.iter().enumerate() {
            if t.0 == num.0 {
                let a = new_nums.remove(i);
                let mut new_index = i as i64 + t.1;
                new_index = new_index.rem_euclid(n as i64 - 1);
                new_nums.insert(new_index as usize, a);
                break;
            }
        }
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 20);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

fn parse_input(input: &str) -> Vec<(usize, i64)> {
    input
        .lines()
        .enumerate()
        .map(|(i, s)| (i, s.parse::<i64>().unwrap()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_one(&input), Some(3));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_two(&input), Some(1623178306));
    }
}
