use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
};

pub fn part_one(input: &str) -> Option<u32> {
    let climbing = Climbing::from(input);
    Some(bfs(&climbing.hill, climbing.start, climbing.end))
}

pub fn part_two(input: &str) -> Option<u32> {
    let climbing = Climbing::from(input);
    let mut ans = u32::MAX;
    for (i, row) in climbing.hill.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c == 0 {
                let step = bfs(&climbing.hill, Position { x: i, y: j }, climbing.end);
                ans = ans.min(step);
            }
        }
    }
    Some(ans)
}

fn bfs(hill: &Vec<Vec<u32>>, start: Position, end: Position) -> u32 {
    let (m, n) = (hill.len(), hill[0].len());
    let mut queue = BinaryHeap::new();
    let mut visited = HashSet::new();
    queue.push(PositionWithStep(start, 0));
    while let Some(PositionWithStep(pos, steps)) = queue.pop() {
        if pos == end {
            return steps;
        }
        for &dir in &[(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let (next_x, next_y) = (pos.x as i32 + dir.0, pos.y as i32 + dir.1);
            if next_x >= 0 && next_x < m as i32 && next_y >= 0 && next_y < n as i32 {
                let index = next_x as usize * n + next_y as usize;
                if visited.contains(&index) {
                    continue;
                }
                if hill[next_x as usize][next_y as usize] as i32 - hill[pos.x][pos.y] as i32 <= 1 {
                    visited.insert(index);
                    queue.push(PositionWithStep(
                        Position {
                            x: next_x as usize,
                            y: next_y as usize,
                        },
                        steps + 1,
                    ));
                }
            }
        }
    }
    u32::MAX
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
struct PositionWithStep(Position, u32);

impl Ord for PositionWithStep {
    fn cmp(&self, other: &Self) -> Ordering {
        other.1.cmp(&self.1)
    }
}

impl PartialOrd for PositionWithStep {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct Climbing {
    start: Position,
    end: Position,
    hill: Vec<Vec<u32>>,
}

impl From<&str> for Climbing {
    fn from(input: &str) -> Self {
        let lines = input.lines();
        let mut hill = Vec::new();
        let (mut start, mut end) = (Position::default(), Position::default());
        for (i, line) in lines.into_iter().enumerate() {
            let mut row = Vec::new();
            for (j, &c) in line.as_bytes().iter().enumerate() {
                match c {
                    b'S' => {
                        start = Position { x: i, y: j };
                        row.push(0);
                    }
                    b'E' => {
                        end = Position { x: i, y: j };
                        row.push(25);
                    }
                    c => row.push((c - b'a') as u32),
                }
            }
            hill.push(row);
        }
        Self { start, end, hill }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(29));
    }
}
