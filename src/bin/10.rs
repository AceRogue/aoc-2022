pub fn part_one(input: &str) -> Option<i32> {
    let instructions = parse_input(input);
    let mut sys = Sys::new();
    for instruction in instructions {
        sys.step(instruction);
    }
    Some(sys.signal_strengths)
}

pub fn part_two(input: &str) -> Option<u32> {
    let instructions = parse_input(input);
    let mut sys = Sys::new();
    for instruction in instructions {
        sys.step(instruction);
    }
    None
}

#[derive(Debug)]
struct Sys {
    x: i32,
    pub cycle: u32,
    pub signal_strengths: i32,
    crt_row: String,
}

impl Sys {
    pub fn new() -> Self {
        Self {
            x: 1,
            cycle: 0,
            signal_strengths: 0,
            crt_row: String::new(),
        }
    }

    pub fn step(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Add(arg) => {
                self.tick();
                self.tick();
                self.x += arg;
            }
            Instruction::Noop => {
                self.tick();
            }
        }
    }

    fn tick(&mut self) {
        self.cycle += 1;
        self.calculate_strength();
        self.draw_crt();
    }

    fn calculate_strength(&mut self) {
        if self.is_cycles() {
            self.signal_strengths += self.x * self.cycle as i32;
        }
    }

    fn is_cycles(&self) -> bool {
        self.cycle == 20 || (self.cycle > 20 && (self.cycle - 20) % 40 == 0)
    }

    fn draw_crt(&mut self) {
        let cur_pos = self.crt_row.len() as i32;
        if cur_pos.abs_diff(self.x) <= 1 {
            self.crt_row.push('#');
        } else {
            self.crt_row.push('.');
        }
        if (cur_pos + 1) % 40 == 0 {
            println!("{}", self.crt_row);
            self.crt_row.clear();
        }
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    Add(i32),
    Noop,
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let op = parts.next().unwrap();
            match op {
                "noop" => Instruction::Noop,
                "addx" => {
                    let arg = parts.next().unwrap().parse().unwrap();
                    Instruction::Add(arg)
                }
                _ => panic!("Unknown instruction"),
            }
        })
        .collect()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), None);
    }
}
