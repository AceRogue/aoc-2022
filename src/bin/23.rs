use std::collections::{HashMap, VecDeque};

pub fn part_one(input: &str) -> Option<i32> {
    let mut grid = Grid::from(input);
    // grid.display();
    grid.simulate(10);
    let res = grid.score();
    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid = Grid::from(input);
    Some(grid.simulate_until_stable())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 23);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn default() -> VecDeque<Self> {
        let mut res = VecDeque::new();
        res.push_back(Direction::North);
        res.push_back(Direction::South);
        res.push_back(Direction::West);
        res.push_back(Direction::East);
        res
    }

    fn forward(&self, pos: (i32, i32)) -> (i32, i32) {
        match self {
            Self::North => (pos.0 - 1, pos.1),
            Self::South => (pos.0 + 1, pos.1),
            Self::West => (pos.0, pos.1 - 1),
            Self::East => (pos.0, pos.1 + 1),
        }
    }
    fn check(&self, pos: (i32, i32)) -> Vec<(i32, i32)> {
        match self {
            Self::North => vec![
                (pos.0 - 1, pos.1),
                (pos.0 - 1, pos.1 - 1),
                (pos.0 - 1, pos.1 + 1),
            ],
            Self::South => vec![
                (pos.0 + 1, pos.1),
                (pos.0 + 1, pos.1 - 1),
                (pos.0 + 1, pos.1 + 1),
            ],
            Self::West => vec![
                (pos.0, pos.1 - 1),
                (pos.0 - 1, pos.1 - 1),
                (pos.0 + 1, pos.1 - 1),
            ],
            Self::East => vec![
                (pos.0, pos.1 + 1),
                (pos.0 - 1, pos.1 + 1),
                (pos.0 + 1, pos.1 + 1),
            ],
        }
    }
}

#[derive(Debug)]
struct Elve {
    pos: (i32, i32),
    propose: (i32, i32),
    check_order: VecDeque<Direction>,
}

impl Elve {
    fn surround(&self) -> Vec<(i32, i32)> {
        let mut res = Vec::new();
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                res.push((self.pos.0 + dx, self.pos.1 + dy));
            }
        }
        res
    }
}

#[derive(Debug)]
struct Grid {
    elves: Vec<Elve>,
    proposes: HashMap<(i32, i32), u32>,
}

impl From<&str> for Grid {
    fn from(s: &str) -> Self {
        let elves = s
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .enumerate()
            .flat_map(|(x, line)| {
                line.into_iter()
                    .enumerate()
                    .map(move |(y, c)| ((x as i32, y as i32), c))
            })
            .filter(|(_, c)| *c == '#')
            .map(|(pos, _)| Elve {
                pos,
                propose: (0, 0),
                check_order: Direction::default(),
            })
            .collect::<Vec<_>>();

        Self {
            elves,
            proposes: HashMap::new(),
        }
    }
}

impl Grid {
    fn propose(&mut self) {
        let cur_pos = self
            .elves
            .iter()
            .map(|e| (e.pos, 1))
            .collect::<HashMap<_, _>>();
        let mut propose_map = HashMap::<(i32, i32), u32>::new();
        for elve in &mut self.elves {
            let mut proposed = elve.pos;
            if elve.surround().iter().all(|pos| !cur_pos.contains_key(pos)) {
                elve.propose = proposed;
            } else {
                for dir in elve.check_order.clone().into_iter() {
                    if dir
                        .check(elve.pos)
                        .iter()
                        .all(|pos| !cur_pos.contains_key(pos))
                    {
                        proposed = dir.forward(elve.pos);
                        break;
                    }
                }
            }
            elve.propose = proposed;
            let dir = elve.check_order.pop_front().unwrap();
            elve.check_order.push_back(dir);
            *propose_map.entry(proposed).or_default() += 1;
        }
        self.proposes = propose_map;
    }

    fn move_elves(&mut self) {
        for elve in &mut self.elves {
            if let Some(count) = self.proposes.get(&elve.propose) {
                if *count > 1 {
                    elve.propose = elve.pos;
                }
            }
        }
        for elve in &mut self.elves {
            elve.pos = elve.propose;
        }
        self.proposes.clear();
    }

    fn simulate(&mut self, round: u32) {
        for _ in 0..round {
            self.propose();
            self.move_elves();
            // self.display();
            // println!();
        }
    }

    #[allow(dead_code)]
    fn display(&self) {
        let (x_min, x_max, y_min, y_max) = self.elves.iter().fold(
            (i32::MAX, i32::MIN, i32::MAX, i32::MIN),
            |(x_min, x_max, y_min, y_max), elve| {
                (
                    x_min.min(elve.pos.0),
                    x_max.max(elve.pos.0),
                    y_min.min(elve.pos.1),
                    y_max.max(elve.pos.1),
                )
            },
        );
        for x in x_min..=x_max {
            for y in y_min..=y_max {
                if self.elves.iter().any(|e| e.pos == (x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    fn simulate_until_stable(&mut self) -> u32 {
        let mut count = 0;
        loop {
            self.propose();
            if self.elves.iter().all(|elve| elve.pos == elve.propose) {
                break;
            }
            self.move_elves();
            count += 1;
        }
        count + 1
    }

    fn score(&self) -> i32 {
        let (x_min, x_max, y_min, y_max) = self.elves.iter().fold(
            (i32::MAX, i32::MIN, i32::MAX, i32::MIN),
            |(x_min, x_max, y_min, y_max), elve| {
                (
                    x_min.min(elve.pos.0),
                    x_max.max(elve.pos.0),
                    y_min.min(elve.pos.1),
                    y_max.max(elve.pos.1),
                )
            },
        );
        (x_max - x_min + 1) * (y_max - y_min + 1) - self.elves.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 23);
        assert_eq!(part_one(&input), Some(110));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 23);
        assert_eq!(part_two(&input), Some(20));
    }
}
