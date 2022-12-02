use std::fs;

fn load_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

fn parse_data(string_data: String) -> Vec<Elf> {
    string_data
        .trim()
        .split("\n\n")
        .map(|elf_inventory| {
            let inventory = elf_inventory
                .lines()
                .map(|item| item.parse().unwrap())
                .collect();
            Elf::new(inventory)
        })
        .collect()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Elf {
    inventory: Vec<u32>,
}

impl Elf {
    pub fn new(inventory: Vec<u32>) -> Self {
        Self { inventory }
    }
}

pub fn solve_part_1(file_path: &str) -> Option<u32> {
    let data = load_file(file_path);
    let elves = parse_data(data);
    elves
        .into_iter()
        .map(|elf| elf.inventory.into_iter().sum())
        .max()
}

pub fn solve_part_2(file_path: &str) -> u32 {
    let data = load_file(file_path);
    let elves = parse_data(data);
    let mut elves_inventory = elves
        .into_iter()
        .map(|elf| elf.inventory.into_iter().sum::<u32>())
        .collect::<Vec<_>>();
    elves_inventory.sort();
    elves_inventory.into_iter().rev().take(3).sum()
}

fn part_1(file_path: &str) {
    let result = solve_part_1(file_path);
    println!("Part 1 result: {:?}", result);
}

fn part_2(file_path: &str) {
    let result = solve_part_2(file_path);
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
    fn load_data_function_returns_vector_of_elves() {
        let data = load_file("./resources/test_data.txt");
        let result = parse_data(data);
        assert_eq!(
            result,
            vec![
                Elf::new(vec![1000, 2000, 3000]),
                Elf::new(vec![4000]),
                Elf::new(vec![5000, 6000]),
                Elf::new(vec![7000, 8000, 9000]),
                Elf::new(vec![10000])
            ]
        );
    }

    #[test]
    fn test_part_1() {
        let result = solve_part_1("./resources/test_data.txt");
        assert_eq!(result, Some(24000));
    }

    #[test]
    fn test_part_2() {
        let result = solve_part_2("./resources/test_data.txt");
        assert_eq!(result, 45000);
    }
}
