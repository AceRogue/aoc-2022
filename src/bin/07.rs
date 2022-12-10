use anyhow::anyhow;

pub fn part_one(input: &str) -> Option<u64> {
    let system_input = SystemInput::from(input);
    let root = system_input.build_directory();
    Some(
        root.all_directory_size()
            .iter()
            .filter(|&f| *f < 100_000)
            .sum::<u64>(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let system_input = SystemInput::from(input);
    let root = system_input.build_directory();
    let used = root.size();
    // 70_000_000 - used + deleted >= 30_000_000
    Some(
        *root
            .all_directory_size()
            .iter()
            .filter(|&f| *f >= 30_000_000 + used - 70_000_000)
            .min()
            .unwrap(),
    )
}

#[derive(Clone, Debug)]
struct Command {
    name: String,
    argument: String,
    files: Vec<FileType>,
}

#[derive(Clone, Debug)]
enum FileType {
    File(File),
    Dir(Dir),
}

#[derive(Clone, Debug)]
struct File {
    _name: String,
    size: u64,
}

impl TryFrom<&str> for File {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut parts = value.split_whitespace();
        let size = parts.next().unwrap().parse::<u64>()?;
        let name = parts.next().unwrap().to_string();
        Ok(File { _name: name, size })
    }
}

#[derive(Clone, Debug)]
struct Dir {
    name: String,
    files: Vec<FileType>,
}

impl TryFrom<&str> for Dir {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut parts = value.split_whitespace();
        let dir = parts.next().unwrap().to_string();
        if dir != "dir" {
            return Err(anyhow!("Expected dir, got {}", dir));
        }
        let name = parts.next().unwrap().to_string();
        Ok(Dir {
            name,
            files: vec![],
        })
    }
}
impl Dir {
    fn find_dir(&mut self, path: &[&str]) -> &mut Self {
        let cur = self;
        path.iter().fold(cur, |cur, next| {
            let dir = cur.files.iter_mut().find_map(|f| match f {
                FileType::Dir(d) if d.name == *next => Some(d),
                _ => None,
            });
            dir.unwrap()
        })
    }

    fn size(&self) -> u64 {
        self.files
            .iter()
            .map(|file| match file {
                FileType::File(f) => f.size,
                FileType::Dir(d) => d.size(),
            })
            .sum()
    }

    fn all_directory_size(&self) -> Vec<u64> {
        self.files.iter().fold(vec![], |mut acc, file| {
            match file {
                FileType::File(_) => {}
                FileType::Dir(d) => {
                    acc.push(d.size());
                    acc.extend(d.all_directory_size());
                }
            }
            acc
        })
    }
}

fn parse_command(input: &str) -> Command {
    let mut parts = input.split_whitespace();
    parts.next();
    let name = parts.next().unwrap().to_string();
    let argument = parts.next().unwrap_or("").to_string();
    Command {
        name,
        argument,
        files: vec![],
    }
}

fn parse_file(input: &str) -> FileType {
    Dir::try_from(input)
        .map(FileType::Dir)
        .unwrap_or_else(|_| FileType::File(File::try_from(input).unwrap()))
}

struct SystemInput(Vec<Command>);

impl From<&str> for SystemInput {
    fn from(input: &str) -> Self {
        let mut commands = Vec::new();
        let mut current: Option<Command> = None;

        for line in input.lines() {
            if line.starts_with('$') {
                commands.extend(current.take());
                let command = parse_command(line);
                current = Some(command);
                continue;
            }

            if let Some(current) = &mut current {
                let file = parse_file(line);
                current.files.push(file);
            }
        }
        commands.extend(current.take());
        Self(commands)
    }
}

impl SystemInput {
    fn build_directory(&self) -> Dir {
        let mut root = Dir {
            name: "/".to_string(),
            files: Vec::new(),
        };
        let mut paths = vec![];
        for command in self.0.iter() {
            match command.name.as_str() {
                "cd" => match command.argument.as_str() {
                    ".." => {
                        paths.pop();
                    }
                    "/" => paths.clear(),
                    a => paths.push(a),
                },
                "ls" => {
                    let dir = root.find_dir(&paths);
                    for file in command.files.iter() {
                        dir.files.push(file.clone());
                    }
                }
                _ => unreachable!(),
            }
        }
        root
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }

    #[test]
    fn test_from_input() {
        let input = advent_of_code::read_file("examples", 7);
        let system_input = SystemInput::from(input.as_str());
        assert_eq!(system_input.0.len(), 10);

        let root = system_input.build_directory();
        assert_eq!(root.files.len(), 4);
        println!("{:?}", root);
    }
}
