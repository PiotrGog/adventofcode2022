use std::{collections::HashMap, fs};

fn load_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

fn parse_data(data: String) -> Vec<Command> {
    let mut result = vec![];
    let lines = data.trim().lines().collect::<Vec<_>>();
    let mut idx = 0;
    while let Some(line) = Command::parse(&lines, &mut idx) {
        result.push(line);
    }

    result
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Command {
    Cd { arg: String },
    Ls { output: Vec<String> },
}

impl Command {
    pub fn parse(iter: &Vec<&str>, idx: &mut usize) -> Option<Self> {
        if *idx >= iter.len() {
            return None;
        }
        let line = iter[*idx];
        *idx += 1;

        if Self::is_ls(line) {
            return Some(Self::parse_ls(iter, idx));
        } else if Self::is_cd(line) {
            return Some(Self::parse_cd(line));
        }

        None
    }

    fn parse_ls(iter: &Vec<&str>, idx: &mut usize) -> Self {
        let mut result = vec![];

        while *idx < iter.len() && !iter[*idx].starts_with("$") {
            result.push(iter[*idx].to_owned());
            *idx += 1;
        }

        Self::Ls { output: result }
    }

    fn parse_cd(command: &str) -> Self {
        Self::Cd {
            arg: command.replace("$ cd ", ""),
        }
    }

    fn is_ls(line: &str) -> bool {
        line == "$ ls"
    }

    fn is_cd(line: &str) -> bool {
        line.starts_with("$ cd")
    }
}

fn get_files(commands: Vec<Command>) -> HashMap<Vec<String>, usize> {
    let mut current_dir = vec![];
    let mut files = HashMap::new();
    commands.into_iter().for_each(|command| match command {
        Command::Cd { arg } => match arg.as_str() {
            "/" => current_dir = vec!["/".to_string()],
            ".." => {
                current_dir.pop();
            }
            _ => current_dir.push(arg),
        },
        Command::Ls { output } => output.into_iter().for_each(|ls| {
            let split = ls.trim().split_whitespace().collect::<Vec<_>>();
            if let Some(file_size) = split[0].parse::<usize>().ok() {
                let mut file_path = current_dir.clone();
                file_path.push(split[1].to_owned());
                files.insert(file_path, file_size);
            }
        }),
    });

    files
}

fn directories_size(files: HashMap<Vec<String>, usize>) -> HashMap<Vec<String>, usize> {
    files
        .into_iter()
        .fold(HashMap::new(), |mut directories_size, file| {
            let (mut path, file_size) = file;
            while let Some((_, directory)) = path.split_last() {
                if let Some(dir_size) = directories_size.get_mut(directory) {
                    *dir_size += file_size;
                } else {
                    directories_size.insert(Vec::from(directory), file_size);
                }
                path = Vec::from(directory);
            }
            directories_size
        })
}

fn sum_directories_with_size_at_most(
    directories_size: HashMap<Vec<String>, usize>,
    size: usize,
) -> usize {
    directories_size
        .into_iter()
        .filter_map(|(_, directory_size)| {
            if directory_size < size {
                Some(directory_size)
            } else {
                None
            }
        })
        .sum()
}

fn find_smalest_dir_with_size_at_most(
    directories_size: HashMap<Vec<String>, usize>,
    size: usize,
) -> usize {
    directories_size
        .into_iter()
        .filter_map(|(_, directory_size)| {
            if directory_size >= size {
                Some(directory_size)
            } else {
                None
            }
        })
        .min()
        .unwrap()
}

pub fn solve_part_1(file_name: &str) -> usize {
    let data = load_file(file_name);
    let commands = parse_data(data);
    let files = get_files(commands);
    let directories = directories_size(files);
    sum_directories_with_size_at_most(directories, 100000)
}

pub fn solve_part_2(file_name: &str) -> usize {
    let data = load_file(file_name);
    let commands = parse_data(data);
    let files = get_files(commands);
    let directories = directories_size(files);
    let total_filesystem_size = 70000000;
    let needed_size = 30000000;
    let used_size = directories.get(&vec!["/".to_string()]).unwrap();
    let available_space = total_filesystem_size - used_size;
    find_smalest_dir_with_size_at_most(directories, needed_size - available_space)
}

fn part_1(file_name: &str) {
    let result = solve_part_1(file_name);
    println!("Part 1 result: {:?}", result);
}

fn part_2(file_name: &str) {
    let result = solve_part_2(file_name);
    println!("Part 2 result: {:?}", result);
}

fn main() {
    const FILE_PATH: &str = "./resources/puzzle.txt";
    part_1(FILE_PATH);
    part_2(FILE_PATH);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_test_data() {
        let data = load_file("./resources/test_data.txt");
        let data = parse_data(data);
        assert_eq!(
            data,
            vec![
                Command::Cd {
                    arg: "/".to_string()
                },
                Command::Ls {
                    output: vec![
                        "dir a".to_string(),
                        "14848514 b.txt".to_string(),
                        "8504156 c.dat".to_string(),
                        "dir d".to_string()
                    ]
                },
                Command::Cd {
                    arg: "a".to_string()
                },
                Command::Ls {
                    output: vec![
                        "dir e".to_string(),
                        "29116 f".to_string(),
                        "2557 g".to_string(),
                        "62596 h.lst".to_string()
                    ]
                },
                Command::Cd {
                    arg: "e".to_string()
                },
                Command::Ls {
                    output: vec!["584 i".to_string()]
                },
                Command::Cd {
                    arg: "..".to_string()
                },
                Command::Cd {
                    arg: "..".to_string()
                },
                Command::Cd {
                    arg: "d".to_string()
                },
                Command::Ls {
                    output: vec![
                        "4060174 j".to_string(),
                        "8033020 d.log".to_string(),
                        "5626152 d.ext".to_string(),
                        "7214296 k".to_string()
                    ]
                }
            ]
        )
    }

    #[test]
    fn test_part_1() {
        let result = solve_part_1("./resources/test_data.txt");
        assert_eq!(result, 95437);
    }

    #[test]
    fn test_part_2() {
        let result = solve_part_2("./resources/test_data.txt");
        assert_eq!(result, 24933642);
    }
}
