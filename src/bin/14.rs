pub fn part_one(input: &str) -> Option<u32> {
    let paths = parse_input(input);
    let ((_, max_x), (min_y, max_y)) = get_max_coord(&paths);
    let mut grid = Grid::new(&paths, max_x, min_y, max_y);
    let count = grid.simulate();
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let paths = parse_input(input);
    let ((_, max_x), (min_y, max_y)) = get_max_coord(&paths);
    let mut grid = Grid::new(&paths, max_x + 2, (min_y - 200).max(0), max_y + 200);
    grid.add_bottom();
    let count = grid.simulate();
    Some(count)
}

fn parse_input(input: &str) -> Vec<Path> {
    input.lines().map(Path::from).collect()
}

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<char>>,
    max_x: usize,
    min_y: usize,
    max_y: usize,
}

fn get_max_coord(paths: &[Path]) -> ((i32, i32), (i32, i32)) {
    let ((_, x_max), (y_min, y_max)) = paths.iter().map(|path| path.min_max_cood()).fold(
        ((i32::MAX, i32::MIN), (i32::MAX, i32::MIN)),
        |acc, item| {
            (
                (acc.0 .0.min(item.0 .0), acc.0 .1.max(item.0 .1)),
                (acc.1 .0.min(item.1 .0), acc.1 .1.max(item.1 .1)),
            )
        },
    );
    ((0, x_max), (y_min, y_max))
}

impl Grid {
    fn new(paths: &[Path], x_max: i32, y_min: i32, y_max: i32) -> Self {
        let mut grid = vec![vec!['.'; (y_max - y_min + 1) as usize]; x_max as usize + 1];
        for path in paths.iter() {
            for i in 1..path.0.len() {
                let (x1, x2) = (
                    path.0[i - 1].x.min(path.0[i].x),
                    path.0[i - 1].x.max(path.0[i].x),
                );
                for x in x1..=x2 {
                    let (y1, y2) = (
                        path.0[i - 1].y.min(path.0[i].y),
                        path.0[i - 1].y.max(path.0[i].y),
                    );
                    for y in y1..=y2 {
                        grid[x as usize][(y - y_min) as usize] = '#';
                    }
                }
            }
        }
        Self {
            grid,
            max_x: x_max as usize,
            min_y: y_min as usize,
            max_y: y_max as usize,
        }
    }

    fn add_bottom(&mut self) {
        for i in 0..self.grid[0].len() {
            self.grid[self.max_x][i] = '#';
        }
    }

    fn _display(&self) {
        for line in self.grid.clone() {
            println!("{:?}", line);
        }
    }

    fn simulate(&mut self) -> u32 {
        let mut t = 0;
        loop {
            if self.simulate_one() {
                break;
            }
            t += 1;
        }
        t
    }

    fn simulate_one(&mut self) -> bool {
        let (mut x, mut y) = (0, 500 - self.min_y);
        loop {
            if self.grid[x][y] != '.' {
                return true;
            }
            let next_x = x + 1;
            if next_x > self.max_x {
                return true;
            }
            if self.grid[next_x][y] == '.' {
                x = next_x;
                continue;
            } else {
                let mut next_y = y as i32 - 1;
                if next_y < 0 {
                    return true;
                }

                if self.grid[next_x][next_y as usize] == '.' {
                    x = next_x;
                    y = next_y as usize;
                    continue;
                } else {
                    next_y = y as i32 + 1;
                }

                if next_y > self.max_y as i32 {
                    return true;
                }
                if self.grid[next_x][next_y as usize] == '.' {
                    x = next_x;
                    y = next_y as usize;
                    continue;
                } else {
                    self.grid[x][y] = 'o';
                    return false;
                }
            }
        }
    }
}

#[derive(Debug)]
struct Path(Vec<Coordinate>);

impl From<&str> for Path {
    fn from(s: &str) -> Self {
        let coords = s
            .split("->")
            .map(|item| {
                let mut iter = item.trim().split(',');
                let y = iter.next().unwrap().trim().parse().unwrap();
                let x = iter.next().unwrap().trim().parse().unwrap();
                Coordinate::new(x, y)
            })
            .collect();
        Self(coords)
    }
}

impl Path {
    fn min_max_cood(&self) -> ((i32, i32), (i32, i32)) {
        let (mut min_x, mut max_x) = (i32::MAX, i32::MIN);
        let (mut min_y, mut max_y) = (i32::MAX, i32::MIN);
        for coord in &self.0 {
            min_x = min_x.min(coord.x);
            max_x = max_x.max(coord.x);
            min_y = min_y.min(coord.y);
            max_y = max_y.max(coord.y);
        }
        ((min_x, max_x), (min_y, max_y))
    }
}

#[derive(Debug)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }
}
