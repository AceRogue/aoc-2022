use std::collections::{HashSet, VecDeque};

pub fn part_one(input: &str) -> Option<u32> {
    let cubes = parse_input(input);
    let mut total = 6 * cubes.len() as u32;
    for i in 0..cubes.len() {
        for j in i..cubes.len() {
            if cubes[i].is_adjecent(&cubes[j]) {
                total -= 2;
            }
        }
    }
    Some(total)
}
pub fn minimum_size(nums: Vec<i32>, max_operations: i32) -> i32 {
    let (mut left, mut right) = (1, *nums.iter().max().unwrap());
    while left < right {
        let mid = (left + right) / 2;
        let mut operations = 0;
        for num in nums.iter() {
            operations += (num - 1) / mid;
        }
        if operations <= max_operations {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    left
}
pub fn part_two(input: &str) -> Option<u32> {
    let cubes = parse_input(input);
    let (min_x, min_y, min_z, max_x, max_y, max_z) = cubes.clone().into_iter().fold(
        (i32::MAX, i32::MAX, i32::MAX, i32::MIN, i32::MIN, i32::MIN),
        |(min_x, min_y, min_z, max_x, max_y, max_z), cube| {
            (
                min_x.min(cube.x),
                min_y.min(cube.y),
                min_z.min(cube.z),
                max_x.max(cube.x),
                max_y.max(cube.y),
                max_z.max(cube.z),
            )
        },
    );

    let cube_set: HashSet<_> = cubes.into_iter().collect();
    let mut outside = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(Cube {
        x: min_x - 1,
        y: min_y - 1,
        z: min_z - 1,
    });
    while let Some(cube) = queue.pop_front() {
        for Cube { x, y, z } in cube.adjecents() {
            if x >= min_x - 1
                && x <= max_x + 1
                && y >= min_y - 1
                && y <= max_y + 1
                && z >= min_z - 1
                && z <= max_z + 1
                && !cube_set.contains(&Cube { x, y, z })
                && outside.insert(Cube { x, y, z })
            {
                queue.push_back(Cube { x, y, z });
            }
        }
    }
    let mut ans = 0;
    for cube in cube_set {
        for neighbor in cube.adjecents() {
            if outside.contains(&neighbor) {
                ans += 1;
            }
        }
    }
    Some(ans)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

impl Cube {
    fn is_adjecent(&self, other: &Cube) -> bool {
        let x = (self.x - other.x).abs();
        let y = (self.y - other.y).abs();
        let z = (self.z - other.z).abs();
        x + y + z == 1
    }

    fn adjecents(&self) -> Vec<Self> {
        vec![
            Cube {
                x: self.x - 1,
                y: self.y,
                z: self.z,
            },
            Cube {
                x: self.x + 1,
                y: self.y,
                z: self.z,
            },
            Cube {
                x: self.x,
                y: self.y - 1,
                z: self.z,
            },
            Cube {
                x: self.x,
                y: self.y + 1,
                z: self.z,
            },
            Cube {
                x: self.x,
                y: self.y,
                z: self.z - 1,
            },
            Cube {
                x: self.x,
                y: self.y,
                z: self.z + 1,
            },
        ]
    }
}

impl From<&str> for Cube {
    fn from(s: &str) -> Self {
        let nums = s
            .split(',')
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        Self {
            x: nums[0],
            y: nums[1],
            z: nums[2],
        }
    }
}

fn parse_input(input: &str) -> Vec<Cube> {
    input.lines().map(Cube::from).collect()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 18);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_one(&input), Some(64));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_two(&input), Some(58));
    }
}
