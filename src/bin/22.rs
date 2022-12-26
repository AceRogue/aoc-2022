pub fn part_one(input: &str) -> Option<i64> {
    let (mut grid, commands) = parse_input(input);
    for command in commands {
        grid.step(&command, 1);
    }
    Some((grid.cur_pos.0 as i64 + 1) * 1000 + (grid.cur_pos.1 as i64 + 1) * 4 + grid.facing.score())
}

pub fn part_two(input: &str) -> Option<i64> {
    let (mut grid, commands) = parse_input(input);
    for command in commands {
        grid.step(&command, 2);
    }
    Some((grid.cur_pos.0 as i64 + 1) * 1000 + (grid.cur_pos.1 as i64 + 1) * 4 + grid.facing.score())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 22);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

// #[derive(Debug, Clone)]
// enum Face {
//     Front,
//     Back,
//     Top,
//     Bottom,
//     Left,
//     Right,
// }

#[derive(Debug, Clone, Copy)]
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
    cube_size: i32,
}

impl Grid {
    fn step(&mut self, command: &Command, part: u32) {
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
                    self.cur_pos = self.move_direction(self.cur_pos, &self.facing.clone(), part);
                }
            }
        }
    }

    fn move_direction(&mut self, cur: (i32, i32), dir: &Direction, part: u32) -> (i32, i32) {
        let mut new_pos = cur;

        loop {
            new_pos = move_direction(new_pos, dir);
            let mut new_dir = *dir;
            if part == 1 {
                new_pos = self.wrap_around_part1(new_pos);
            } else {
                (new_pos, new_dir) = self.wrap_around_part2(new_pos, dir);
            }

            if self.grid[new_pos.0 as usize][new_pos.1 as usize] == '#' {
                let ops = self.facing.oppsite();
                new_pos = self.move_direction(new_pos, &ops, part);
                self.facing = *dir;
                break;
            }
            if self.grid[new_pos.0 as usize][new_pos.1 as usize] == '.' {
                self.facing = new_dir;
                break;
            }
        }
        new_pos
    }

    fn wrap_around_part1(&self, cur: (i32, i32)) -> (i32, i32) {
        let mut new_pos = cur;
        if new_pos.0 < 0 {
            new_pos.0 += self.grid.len() as i32;
        } else if new_pos.0 >= self.grid.len() as i32 {
            new_pos.0 -= self.grid.len() as i32
        } else if new_pos.1 < 0 {
            new_pos.1 += self.grid[new_pos.0 as usize].len() as i32;
        } else if new_pos.1 >= self.grid[new_pos.0 as usize].len() as i32 {
            new_pos.1 -= self.grid[new_pos.0 as usize].len() as i32;
        }
        new_pos
    }

    fn wrap_around_part2(
        &mut self,
        mut cur: (i32, i32),
        dir: &Direction,
    ) -> ((i32, i32), Direction) {
        /*  hard code state transition..
        _ 1 2
        _ 3 _
        4 5 _
        6 _ _
        */
        if cur.0 < 0 {
            cur.0 += 1;
        } else if cur.0 >= self.grid.len() as i32 {
            cur.0 -= 1
        } else if cur.1 < 0 {
            cur.1 += 1;
        } else if cur.1 >= self.grid[cur.0 as usize].len() as i32 {
            cur.1 -= 1;
        } else if self.grid[cur.0 as usize][cur.1 as usize] != ' ' {
            return (cur, *dir);
        }

        let (new_row, new_col, new_facing) =
            match (cur.0 / self.cube_size, cur.1 / self.cube_size, dir) {
                (0, 0, Direction::Left) => (2, 0, Direction::Right),
                (0, 1, Direction::Up) => (3, 0, Direction::Right),
                (0, 2, Direction::Up) => (3, 0, Direction::Up),
                (0, 2, Direction::Right) => (2, 1, Direction::Left),
                (1, 2, Direction::Down) => (1, 1, Direction::Left),
                (1, 0, Direction::Left) => (2, 0, Direction::Down),
                (1, 2, Direction::Right) => (0, 2, Direction::Up),
                (2, 2, Direction::Right) => (0, 2, Direction::Left),
                (3, 1, Direction::Down) => (3, 0, Direction::Left),
                (2, 0, Direction::Left) => (0, 1, Direction::Right),
                (1, 0, Direction::Up) => (1, 1, Direction::Right),
                (3, 0, Direction::Left) => (0, 1, Direction::Down),
                (3, 1, Direction::Right) => (2, 1, Direction::Up),
                (3, 0, Direction::Down) => (0, 2, Direction::Down),
                _ => unreachable!(),
            };
        let (dr, dc) = (cur.0 % self.cube_size, cur.1 % self.cube_size);
        let t = match dir {
            Direction::Up => dc,
            Direction::Down => self.cube_size - dc - 1,
            Direction::Left => self.cube_size - dr - 1,
            Direction::Right => dr,
        };
        let (tr, tc) = match new_facing {
            Direction::Up => (self.cube_size - 1, t),
            Direction::Down => (0, self.cube_size - t - 1),
            Direction::Left => (self.cube_size - t - 1, self.cube_size - 1),
            Direction::Right => (t, 0),
        };
        self.facing = new_facing;
        let new_pos = (new_row * self.cube_size + tr, new_col * self.cube_size + tc);
        // println!("origin_pos: {:?}, origin_facing: {:?}", cur, dir);
        // println!("new_pos: {:?}, new_facing: {:?}", new_pos, new_facing);
        (new_pos, new_facing)
    }

    // fn build_face_transitions(&mut self) {
    //     // face positions
    //     let faces = self.faces();
    //     let mut transitions = HashMap::new();

    // }

    #[allow(dead_code)]
    fn faces(&self) -> Vec<(usize, usize)> {
        let mut ans = vec![];
        for i in 0..6 {
            for j in 0..6 {
                if self.grid[i * self.cube_size as usize][j * self.cube_size as usize] != ' ' {
                    ans.push((i, j))
                }
            }
        }
        ans
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

    let mut cube_size = i32::MAX;
    for row in grid.iter_mut() {
        while row.len() < max_column as usize {
            row.push(' ');
        }
        cube_size = cube_size.min(row.iter().filter(|&c| *c == ' ').count() as i32);
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
        cube_size,
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
    }
}
