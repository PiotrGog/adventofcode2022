use std::{fs, str::FromStr, vec};

fn load_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

fn parse_data(data: String) -> Vec<Operation> {
    data.trim()
        .lines()
        .map(|line| Operation::from_str(line).unwrap())
        .collect::<Vec<_>>()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Operation {
    Noop,
    Add(isize),
}

impl FromStr for Operation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "noop" => Ok(Self::Noop),
            x if x.starts_with("addx ") => Ok(Self::Add(x.replace("addx ", "").parse().unwrap())),
            x => Err(format!("Unsupported command \"{x}\"")),
        }
    }
}

impl Operation {
    fn convert_multi_to_single_cycle_instruction(self) -> Vec<Self> {
        match self {
            Self::Noop => vec![Self::Noop],
            Self::Add(x) => vec![Self::Noop, Self::Add(x)],
        }
    }
}

fn convert_all_operations_to_single_cycle_instructions(
    operations: Vec<Operation>,
) -> Vec<Operation> {
    operations.into_iter().fold(vec![], |mut acc, operation| {
        acc.append(&mut operation.convert_multi_to_single_cycle_instruction());
        acc
    })
}

fn calculate_signal_strength_sum(
    mut operations: Vec<Operation>,
    chunk_size: usize,
    indexes: &[usize],
) -> isize {
    operations.insert(0, Operation::Noop);
    let signals_strength = convert_all_operations_to_single_cycle_instructions(operations)
        .chunks(chunk_size)
        .fold(vec![1], |mut x_values, sub_operations| {
            let sum = sub_operations
                .iter()
                .map(|operation| match operation {
                    Operation::Noop => 0,
                    Operation::Add(x) => *x,
                })
                .sum::<isize>()
                + x_values.last().unwrap();
            x_values.push(sum);
            x_values
        });
    indexes
        .into_iter()
        .map(|index| signals_strength[index / chunk_size] * *index as isize)
        .sum()
}

fn solve_part_1(file_path: &str) -> isize {
    let data = load_file(file_path);
    let operations = parse_data(data);
    calculate_signal_strength_sum(operations, 20, &[20, 60, 100, 140, 180, 220])
}

fn part_1(file_path: &str) {
    let result = solve_part_1(file_path);
    println!("Part 1 result: {:?}", result);
}

fn main() {
    const FILE_PATH: &str = "./resources/puzzle.txt";
    part_1(FILE_PATH);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let result = solve_part_1("./resources/test_data.txt");
        assert_eq!(result, 13140);
    }
}
