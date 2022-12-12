use std::cmp;

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_input(input);
    let (m, n) = (grid.len(), grid[0].len());

    let mut heighest_bottom_right = vec![vec![(0, 0); n]; m];
    for i in (1..m - 1).rev() {
        for j in (1..n - 1).rev() {
            heighest_bottom_right[i][j].0 =
                cmp::max(heighest_bottom_right[i + 1][j].0, grid[i + 1][j]);
            heighest_bottom_right[i][j].1 =
                cmp::max(heighest_bottom_right[i][j + 1].1, grid[i][j + 1]);
        }
    }
    let mut heighest_top_left = vec![vec![(0, 0); n]; m];
    let mut ans = 2 * (m + n) as u32 - 4;
    for i in 1..m - 1 {
        for j in 1..n - 1 {
            heighest_top_left[i][j].0 = cmp::max(heighest_top_left[i - 1][j].0, grid[i - 1][j]);
            heighest_top_left[i][j].1 = cmp::max(heighest_top_left[i][j - 1].1, grid[i][j - 1]);
            let cur = grid[i][j];
            if cur > heighest_top_left[i][j].0
                || cur > heighest_top_left[i][j].1
                || cur > heighest_bottom_right[i][j].0
                || cur > heighest_bottom_right[i][j].1
            {
                ans += 1;
            }
        }
    }
    Some(ans)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_input(input);
    let (m, n) = (grid.len(), grid[0].len());
    let mut ans = 0;
    for i in 1..m - 1 {
        for j in 1..n - 1 {
            let down = get_distance(&grid, (i, j), (1, 0));
            let up = get_distance(&grid, (i, j), (-1, 0));
            let left = get_distance(&grid, (i, j), (0, -1));
            let right = get_distance(&grid, (i, j), (0, 1));
            ans = ans.max(left * right * up * down);
        }
    }
    Some(ans)
}

fn get_distance(grid: &Vec<Vec<u32>>, pos: (usize, usize), dirction: (i32, i32)) -> u32 {
    let (x, y) = pos;
    let (mut x, mut y) = (x as i32, y as i32);
    let (dx, dy) = dirction;
    let cur = grid[x as usize][y as usize];
    let mut ans = 0;
    while x + dx < grid.len() as i32 && y + dy < grid[0].len() as i32 && x + dx >= 0 && y + dy >= 0
    {
        let next = grid[(x + dx) as usize][(y + dy) as usize];
        if cur > next {
            ans += 1;
            x += dx;
            y += dy;
        } else {
            ans += 1;
            break;
        }
    }
    ans
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.bytes()
                .into_iter()
                .map(|c| (c - b'0') as u32)
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
