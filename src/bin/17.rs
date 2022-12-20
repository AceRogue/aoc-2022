use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u64> {
    let jet = Jet::from(input);
    let mut chamber = Chamber::new(jet);
    let rocks = rocks();

    for i in 0..2022 {
        chamber.fall_rock(&rocks[i % rocks.len()]);
    }

    Some(chamber.cur_height as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let jet = Jet::from(input);
    let mut chamber = Chamber::new(jet);
    let rocks = rocks();
    let mut seen = HashMap::new();
    let total = 1_000_000_000_000;
    let mut heights = vec![];
    for i in 0..total {
        let rock_index = i % rocks.len();
        let jet_index = chamber.jet.cur;
        chamber.fall_rock(&rocks[rock_index]);
        let snap = chamber.snapshot();
        if let Some(last_index) = seen.insert((jet_index, rock_index, snap), i) {
            let cycle = i - last_index;
            let n = (total - i) / cycle;
            let m = (total - i) % cycle;
            println!(
                "rock_index: {}, jet_index: {}, cycle: {:?}, m: {}, n: {}",
                rock_index, jet_index, cycle, m, n
            );
            println!(
                "last_index: {}, last_height: {}",
                last_index, heights[last_index]
            );
            return Some(
                heights[m + last_index - 1] as u64
                    + (n + 1) as u64 * (chamber.cur_height - heights[last_index]) as u64,
            );
        }
        heights.push(chamber.cur_height);
    }

    None
}

#[derive(Debug)]
struct Rock {
    pub blocks: Vec<(i64, i64)>,
}

fn rocks() -> Vec<Rock> {
    vec![
        Rock {
            blocks: vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        },
        Rock {
            blocks: vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)],
        },
        Rock {
            blocks: vec![(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)],
        },
        Rock {
            blocks: vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        },
        Rock {
            blocks: vec![(0, 0), (0, 1), (1, 0), (1, 1)],
        },
    ]
}

#[derive(Debug)]
struct Chamber {
    grid: Vec<Vec<char>>,
    cur_height: i64,
    jet: Jet,
}

impl Chamber {
    fn new(jet: Jet) -> Self {
        Self {
            grid: vec![vec!['.'; 7]; 30000],
            cur_height: 0,
            jet,
        }
    }

    fn snapshot(&self) -> String {
        let mut s = String::new();
        for row in (self.cur_height - 30).max(0)..self.cur_height {
            for col in 0..7 {
                s.push(self.grid[row as usize][col]);
            }
        }
        s
    }

    fn fall_rock(&mut self, rock: &Rock) {
        let mut start_x = self.cur_height + 3;
        let mut start_y = 2;
        loop {
            let jet = self.jet.next();
            if self.can_move(rock, (start_x, start_y), (0, jet as i64)) {
                start_y += jet as i64;
            }
            if self.can_move(rock, (start_x, start_y), (-1, 0)) {
                start_x -= 1;
            } else {
                // can not move
                self.integrate_rock(rock, (start_x, start_y));
                break;
            }
        }
    }

    fn can_move(&self, rock: &Rock, start_pos: (i64, i64), diff: (i64, i64)) -> bool {
        rock.blocks.iter().all(|(x, y)| {
            let (new_x, new_y) = (start_pos.0 + diff.0 + x, start_pos.1 + diff.1 + y);
            (0..7).contains(&new_y)
                && new_x >= 0
                && self.grid[new_x as usize][new_y as usize] != '#'
        })
    }

    fn integrate_rock(&mut self, rock: &Rock, start_pos: (i64, i64)) {
        for (x, y) in rock.blocks.iter() {
            let x = x + start_pos.0;
            let y = y + start_pos.1;
            self.grid[x as usize][y as usize] = '#';
            self.cur_height = self.cur_height.max(x + 1);
        }
    }
}

#[derive(Debug)]
struct Jet {
    data: Vec<i32>,
    cur: usize,
}

impl From<&str> for Jet {
    fn from(s: &str) -> Self {
        let data = s
            .as_bytes()
            .iter()
            .map(|&c| if c == b'>' { 1 } else { -1 })
            .collect();
        Self { data, cur: 0 }
    }
}

impl Jet {
    fn next(&mut self) -> i32 {
        let c = self.data[self.cur];
        self.cur += 1;
        self.cur %= self.data.len();
        c
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 17);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_one(&input), Some(3068));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_two(&input), Some(1514285714288));
    }
}
