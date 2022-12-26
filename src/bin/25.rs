pub fn part_one(input: &str) -> Option<String> {
    let sum: i64 = input.lines().map(|line| Snafu(line.to_string())).map(Into::<i64>::into).sum();

    Some(Snafu::new(sum).0)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 25);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

struct Snafu(String);

impl Snafu {
    fn new(mut value: i64) -> Self {
        let mut res = vec![];
        let mut base = i64::pow(5, 20);
        while base > 0 {
            let digit = value / base;
            res.push(digit);
            value %= base; 
            base /= 5;
        }
        let mut ans = String::new();
        let mut addon = 0;
        for &num in res.iter().rev() {
            let num = num + addon;
            addon = 0;
            match num {
                0 => ans.push('0'),
                1 => ans.push('1'),
                2 => ans.push('2'),
                3 => {
                    ans.push('=');
                    addon = 1;
                }
                4 => {
                    ans.push('-');
                    addon = 1;
                }
                5 => {
                    ans.push('0');
                    addon = 1;
                }
                _ => unreachable!(),
            }
            
        }
        let mut ans = ans.chars().rev().collect::<String>();
        while ans.starts_with('0') {
            ans.remove(0);
        }
        Self(ans)
    }
}

impl From<Snafu> for i64 {
    fn from(value: Snafu) -> Self {
        let mut ans = 0;
        let mut base = 1;
        for c in value.0.chars().rev() {
            match c {
                '0' => {}
                '1' => ans += base,
                '2' => ans += base * 2,
                '-' => ans -= base,
                '=' => ans -= base * 2,
                _ => unreachable!(),
            }
            base *= 5;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 25);
        assert_eq!(part_one(&input), Some("2=-1=0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 25);
        assert_eq!(part_two(&input), None);
    }
}
