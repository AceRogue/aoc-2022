use lazy_static::lazy_static;

pub fn part_one(input: &str) -> Option<u32> {
    let pairs = parse_input(input);
    Some(count_unavailable_positions(&pairs, 2000000))
}

pub fn part_two(input: &str) -> Option<u64> {
    let pairs = parse_input(input);
    // let (x, y) = find_available_position(&pairs, 20);
    let (x, y) = find_available_position(&pairs, 4000000);
    Some(x as u64 * 4000000 + y as u64)
}

lazy_static!(
    static ref RE: regex::Regex = regex::Regex::new(r"(?x)Sensor\sat\sx=(?P<sensor_x>-?\d+),\sy=(?P<sensor_y>-?\d+):\sclosest\sbeacon\sis\sat\sx=(?P<beacon_x>-?\d+),\sy=(?P<beacon_y>-?\d+)").unwrap();
);

fn count_unavailable_positions(pairs: &[SensorBeaconPair], y: i32) -> u32 {
    let mut set = std::collections::HashSet::new();
    let mut beacons = std::collections::HashSet::new();
    for pair in pairs {
        beacons.insert((pair.beacon_x, pair.beacon_y));
    }

    for pair in pairs {
        let diff_y = pair.sensor_y.abs_diff(y);
        if diff_y > pair.distance {
            continue;
        }
        for x in (pair.sensor_x - (pair.distance - diff_y) as i32)
            ..=(pair.sensor_x + (pair.distance - diff_y) as i32)
        {
            if beacons.contains(&(x, y)) {
                continue;
            }
            set.insert(x);
        }
    }
    set.len() as u32
}

fn find_available_position(pairs: &[SensorBeaconPair], limit: i32) -> (i32, i32) {
    let mut beacons = std::collections::HashSet::new();
    for pair in pairs {
        beacons.insert((pair.beacon_x, pair.beacon_y));
    }

    for y in 0..=limit {
        let mut intervals = Vec::new();
        for pair in pairs {
            let diff_y = pair.sensor_y.abs_diff(y);
            let diff = pair.distance as i32 - diff_y as i32;
            if diff < 0 {
                continue;
            }
            let start_x = (pair.sensor_x - diff).max(0);
            let end_x = (pair.sensor_x + diff).min(limit);
            intervals.push((start_x, end_x));
        }
        merge_intervals(&mut intervals);
        if intervals.len() > 1 {
            return (intervals[0].1 + 1, y);
        }
    }
    unreachable!()
}

fn merge_intervals(intervals: &mut Vec<(i32, i32)>) {
    intervals.sort_by_key(|(start, _)| *start);
    let mut i = 0;
    while i < intervals.len() - 1 {
        let (start, end) = intervals[i];
        let (next_start, next_end) = intervals[i + 1];
        if next_start <= end {
            intervals[i] = (start, end.max(next_end));
            intervals.remove(i + 1);
        } else {
            i += 1;
        }
    }
}


#[derive(Debug)]
struct SensorBeaconPair {
    sensor_x: i32,
    sensor_y: i32,
    beacon_x: i32,
    beacon_y: i32,
    distance: u32,
}

impl From<&str> for SensorBeaconPair {
    fn from(s: &str) -> Self {
        let caps = RE.captures(s).unwrap();
        let sensor_x = caps["sensor_x"].parse::<i32>().unwrap();
        let sensor_y = caps["sensor_y"].parse::<i32>().unwrap();
        let beacon_x = caps["beacon_x"].parse::<i32>().unwrap();
        let beacon_y = caps["beacon_y"].parse::<i32>().unwrap();
        let distance = sensor_x.abs_diff(beacon_x) + sensor_y.abs_diff(beacon_y);
        Self {
            sensor_x,
            sensor_y,
            beacon_x,
            beacon_y,
            distance,
        }
    }
}

fn parse_input(input: &str) -> Vec<SensorBeaconPair> {
    input.lines().map(SensorBeaconPair::from).collect()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_one(&input), Some(0));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two(&input), Some(56000011));
    }
}
