use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    let commands = parse_input(input);
    let mut set = HashSet::new();
    let mut rope = Rope::new(1);
    for command in commands {
        for _ in 0..command.steps {
            let (tail_x, tail_y) = rope.step(&command.direction);
            set.insert((tail_x, tail_y));
        }
    }
    Some(set.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let commands = parse_input(input);
    let mut set = HashSet::new();
    let mut rope = Rope::new(9);
    for command in commands {
        for _ in 0..command.steps {
            let (tail_x, tail_y) = rope.step(&command.direction);
            set.insert((tail_x, tail_y));
        }
    }
    Some(set.len() as u32)
}

#[derive(Debug, Clone, Default)]
struct Knot {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
struct Rope {
    pub knots: Vec<Knot>,
    nums: u32,
}

impl Rope {
    pub fn new(nums: u32) -> Self {
        Self {
            knots: vec![Knot { x: 5000, y: 5000 }; nums as usize + 1],
            nums,
        }
    }

    fn step(&mut self, direction: &Direction) -> (i32, i32) {
        match direction {
            Direction::Left => self.knots[0].x -= 1,
            Direction::Right => self.knots[0].x += 1,
            Direction::Up => self.knots[0].y += 1,
            Direction::Down => self.knots[0].y -= 1,
        }
        self.step_tails();

        (
            self.knots[self.nums as usize].x,
            self.knots[self.nums as usize].y,
        )
    }

    fn step_tails(&mut self) {
        for i in 1..=self.nums {
            self.step_tail(i as usize);
        }
    }

    fn step_tail(&mut self, index: usize) {
        if !self.is_adjacent(index) {
            let diff = (self.knots[index].x - self.knots[index - 1].x, self.knots[index].y - self.knots[index - 1].y);
            let l = diff.0.abs().max(diff.1.abs());
            let m = (diff.0 / l, diff.1 / l);
            self.knots[index].x = self.knots[index-1].x + m.0;
            self.knots[index].y = self.knots[index-1].y + m.1;
        }
    }

    fn is_adjacent(&self, index: usize) -> bool {
        (self.knots[index].x - self.knots[index - 1].x).abs() <= 1
            && (self.knots[index].y - self.knots[index - 1].y).abs() <= 1
    }
}

fn parse_input(input: &str) -> Vec<Command> {
    input.lines().map(Command::from).collect()
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl From<&str> for Direction {
    fn from(s: &str) -> Self {
        match s {
            "L" => Direction::Left,
            "R" => Direction::Right,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _ => panic!("Invalid direction"),
        }
    }
}

#[derive(Debug)]
struct Command {
    pub direction: Direction,
    pub steps: u32,
}

impl From<&str> for Command {
    fn from(s: &str) -> Self {
        let mut parts = s.split_whitespace();
        let direction = Direction::from(parts.next().unwrap());
        let steps = parts.next().unwrap().parse::<u32>().unwrap();
        Self { direction, steps }
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(36));
    }
}
