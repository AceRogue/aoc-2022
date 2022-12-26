use std::collections::{HashSet, VecDeque};

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid = Grid::from(input);
    let step = grid.schedule(grid.start, grid.end);
    Some(step)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut ans = 0;
    let mut grid = Grid::from(input);
    ans += grid.schedule(grid.start, grid.end);
    ans += grid.schedule(grid.end, grid.start);
    ans += grid.schedule(grid.start, grid.end);
    Some(ans)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 24);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn step(&self, pos: (i32, i32)) -> (i32, i32) {
        match self {
            Direction::Up => (pos.0 - 1, pos.1),
            Direction::Down => (pos.0 + 1, pos.1),
            Direction::Left => (pos.0, pos.1 - 1),
            Direction::Right => (pos.0, pos.1 + 1),
        }
    }
}

#[derive(Debug)]
struct Blizzard {
    pos: (i32, i32),
    direction: Direction,
}

impl Blizzard {
    fn step(&mut self) {
        self.pos = self.direction.step(self.pos);
    }
}

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<char>>,
    blizzards: Vec<Blizzard>,
    start: (i32, i32),
    end: (i32, i32),
}

impl Grid {
    fn schedule(&mut self, start: (i32, i32), end: (i32, i32)) -> u32 {
        let mut steps = 0;
        let mut queue = VecDeque::new();
        queue.push_back(start);
        'outer: loop {
            steps += 1;
            self.blizzards_move();
            let blizzards = self.blizzards.iter().map(|b| b.pos).collect::<HashSet<_>>();
            let mut new_queue = HashSet::new();
            for pos in queue.iter() {
                for dir in vec![
                    Direction::Up,
                    Direction::Down,
                    Direction::Left,
                    Direction::Right,
                ] {
                    let new_pos = dir.step(*pos);
                    if new_pos == end {
                        break 'outer;
                    }
                    if self.valid_index(new_pos)
                        && self.grid[new_pos.0 as usize][new_pos.1 as usize] != '#'
                        && !blizzards.contains(&new_pos)
                    {
                        new_queue.insert(new_pos);
                    }
                }
                if !blizzards.contains(pos) {
                    new_queue.insert(*pos);
                }
            }
            queue = new_queue.into_iter().collect::<VecDeque<_>>();
        }
        steps
    }

    fn valid_index(&self, pos: (i32, i32)) -> bool {
        pos.0 >= 0
            && pos.0 < self.grid.len() as i32
            && pos.1 >= 0
            && pos.1 < self.grid[0].len() as i32
    }

    fn blizzards_move(&mut self) {
        for blizzard in &mut self.blizzards {
            blizzard.step();
            if self.grid[blizzard.pos.0 as usize][blizzard.pos.1 as usize] == '#' {
                match blizzard.direction {
                    Direction::Up => {
                        blizzard.pos =
                            (blizzard.pos.0 + self.grid.len() as i32 - 2, blizzard.pos.1);
                    }
                    Direction::Down => {
                        blizzard.pos =
                            (blizzard.pos.0 - self.grid.len() as i32 + 2, blizzard.pos.1);
                    }
                    Direction::Left => {
                        blizzard.pos = (
                            blizzard.pos.0,
                            blizzard.pos.1 + self.grid[0].len() as i32 - 2,
                        );
                    }
                    Direction::Right => {
                        blizzard.pos = (
                            blizzard.pos.0,
                            blizzard.pos.1 - self.grid[0].len() as i32 + 2,
                        );
                    }
                }
            }
        }
    }
}

impl From<&str> for Grid {
    fn from(s: &str) -> Self {
        let mut grid = Vec::new();
        let mut blizzards = Vec::new();
        let mut start = (0, 0);
        let mut end = (0, 0);
        for (x, line) in s.lines().enumerate() {
            let mut row = Vec::new();
            for (y, c) in line.chars().enumerate() {
                if x == 0 && c == '.' {
                    start = (x as i32, y as i32);
                } else if x == s.lines().count() - 1 && c == '.' {
                    end = (x as i32, y as i32);
                }
                match c {
                    '>' => {
                        blizzards.push(Blizzard {
                            pos: (x as i32, y as i32),
                            direction: Direction::Right,
                        });
                    }
                    '<' => {
                        blizzards.push(Blizzard {
                            pos: (x as i32, y as i32),
                            direction: Direction::Left,
                        });
                    }
                    '^' => {
                        blizzards.push(Blizzard {
                            pos: (x as i32, y as i32),
                            direction: Direction::Up,
                        });
                    }
                    'v' => {
                        blizzards.push(Blizzard {
                            pos: (x as i32, y as i32),
                            direction: Direction::Down,
                        });
                    }
                    _ => {}
                }
                row.push(c);
            }
            grid.push(row);
        }
        Grid {
            grid,
            blizzards,
            start,
            end,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 24);
        assert_eq!(part_one(&input), Some(18));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 24);
        assert_eq!(part_two(&input), Some(54));
    }
}
