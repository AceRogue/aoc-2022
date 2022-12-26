pub fn part_one(input: &str) -> Option<i64> {
    let (mut grid, commands) = parse_input(input);
    for command in commands {
        grid.step(&command);
    }
    Some((grid.cur_pos.0 as i64 + 1) * 1000 + (grid.cur_pos.1 as i64 + 1) * 4 + grid.facing.score())
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 22);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn oppsite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    fn score(&self) -> i64 {
        match self {
            Direction::Up => 3,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Right => 0,
        }
    }
}

fn move_direction(cur: (i32, i32), dir: &Direction) -> (i32, i32) {
    match dir {
        Direction::Up => (cur.0 - 1, cur.1),
        Direction::Down => (cur.0 + 1, cur.1),
        Direction::Left => (cur.0, cur.1 - 1),
        Direction::Right => (cur.0, cur.1 + 1),
    }
}

#[derive(Debug, Clone)]
struct Grid {
    grid: Vec<Vec<char>>,
    facing: Direction,
    cur_pos: (i32, i32),
}

impl Grid {
    fn step(&mut self, command: &Command) {
        // println!(
        // "command: {:?}, pos: {:?}, facing: {:?}",
        // command, self.cur_pos, self.facing
        // );
        match command {
            Command::TurnLeft => {
                self.facing = match self.facing {
                    Direction::Up => Direction::Left,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Down,
                    Direction::Right => Direction::Up,
                }
            }
            Command::TurnRight => {
                self.facing = match self.facing {
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                }
            }
            Command::Move(num) => {
                for _ in 0..*num {
                    self.cur_pos = self.move_direction(self.cur_pos, &self.facing.clone());
                }
            }
        }
    }

    fn move_direction(&self, cur: (i32, i32), dir: &Direction) -> (i32, i32) {
        let mut new_pos = cur;

        loop {
            // while self.grid[new_pos.0 as usize][new_pos.1 as usize] != '.' {
            new_pos = move_direction(new_pos, dir);
            // println!("cur: {:?}, dir: {:?}", new_pos, dir);
            if new_pos.0 < 0 {
                new_pos.0 += self.grid.len() as i32;
            } else if new_pos.0 >= self.grid.len() as i32 {
                new_pos.0 -= self.grid.len() as i32
            } else if new_pos.1 < 0 {
                new_pos.1 += self.grid[new_pos.0 as usize].len() as i32;
            } else if new_pos.1 >= self.grid[new_pos.0 as usize].len() as i32 {
                new_pos.1 -= self.grid[new_pos.0 as usize].len() as i32;
            }

            if self.grid[new_pos.0 as usize][new_pos.1 as usize] == '#' {
                let ops = self.facing.oppsite();
                new_pos = self.move_direction(new_pos, &ops);
                break;
            }
            if self.grid[new_pos.0 as usize][new_pos.1 as usize] == '.' {
                break;
            }
        }
        new_pos
    }
}

#[derive(Debug, Clone)]
enum Command {
    TurnLeft,
    TurnRight,
    Move(i32),
}

fn parse_input(input: &str) -> (Grid, Vec<Command>) {
    let mut grid = Vec::new();
    let mut lines = input.lines();
    let mut max_column = 0;
    // build map
    for line in lines.by_ref() {
        let mut row = Vec::new();
        if line.is_empty() {
            break;
        }
        for c in line.chars() {
            row.push(c);
        }
        max_column = max_column.max(row.len() as i32);
        grid.push(row);
    }
    for row in grid.iter_mut() {
        while row.len() < max_column as usize {
            row.push(' ');
        }
    }

    let mut start_pos = 0;
    for i in 0..grid[0].len() {
        if grid[0][i] == '.' {
            start_pos = i;
            break;
        }
    }
    let grid = Grid {
        grid,
        facing: Direction::Right,
        cur_pos: (0, start_pos as i32),
    };
    // parse command
    let mut commands = Vec::new();
    let row = lines.next().unwrap();
    let mut num = 0;
    for c in row.chars() {
        match c {
            'L' => {
                if num != 0 {
                    commands.push(Command::Move(num));
                    num = 0;
                }
                commands.push(Command::TurnLeft)
            }
            'R' => {
                if num != 0 {
                    commands.push(Command::Move(num));
                    num = 0;
                }
                commands.push(Command::TurnRight)
            }
            c if c.is_alphanumeric() => {
                num = num * 10 + c.to_digit(10).unwrap() as i32;
            }
            _ => unreachable!(),
        }
    }
    if num > 0 {
        commands.push(Command::Move(num));
    }
    (grid, commands)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 22);
        assert_eq!(part_one(&input), Some(6032));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 22);
        assert_eq!(part_two(&input), None);
    }

    #[test]
    fn test_parse_input() {
        let input = advent_of_code::read_file("examples", 22);
        let (grid, commands) = parse_input(&input);
        assert_eq!(grid.grid.len(), 12);
        assert_eq!(commands.len(), 13);
        // println!("{:#?}", grid);
        println!("{:?}", commands);
    }
}
