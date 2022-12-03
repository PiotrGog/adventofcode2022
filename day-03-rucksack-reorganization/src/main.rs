use std::{collections::HashSet, fs};

fn load_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

fn parse_data(string_data: String) -> Vec<Rucksack> {
    string_data
        .trim()
        .lines()
        .map(|line| Rucksack::new(line))
        .collect()
}

pub fn get_item_priority(item: char) -> usize {
    match item {
        'a'..='z' => item as usize - 'a' as usize + 1,
        'A'..='Z' => item as usize - 'A' as usize + 27,
        v => panic!("Can't prioritize item with type {}", v),
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Rucksack {
    items: String,
}

impl Rucksack {
    pub fn new(items: &str) -> Self {
        Self {
            items: items.to_string(),
        }
    }

    pub fn get_the_same_type_in_compartments(&self) -> Option<char> {
        let (item1, item2) = self.items.split_at(self.items.len() / 2);
        let common_letters = item1
            .chars()
            .collect::<HashSet<_>>()
            .intersection(&item2.chars().collect::<HashSet<_>>())
            .cloned()
            .collect::<Vec<_>>();
        if common_letters.len() != 1 {
            return None;
        }
        Some(common_letters[0])
    }
}

pub fn solve_part_1(file_path: &str) -> usize {
    let data = load_file(file_path);
    let rounds = parse_data(data);
    rounds
        .into_iter()
        .map(|rucksack| get_item_priority(rucksack.get_the_same_type_in_compartments().unwrap()))
        .sum()
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
    fn load_data_function_returns_rucksacks() {
        let data = load_file("./resources/test_data.txt");
        let result = parse_data(data);
        assert_eq!(
            result,
            vec![
                Rucksack::new("vJrwpWtwJgWrhcsFMMfFFhFp"),
                Rucksack::new("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
                Rucksack::new("PmmdzqPrVvPwwTWBwg"),
                Rucksack::new("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"),
                Rucksack::new("ttgJtRGJQctTZtZT"),
                Rucksack::new("CrZsJsPPZsGzwwsLwLmpwMDw")
            ]
        );
    }

    #[test]
    fn get_type_from_rucksack_compartments() {
        assert_eq!(
            Rucksack::new("vJrwpWtwJgWrhcsFMMfFFhFp").get_the_same_type_in_compartments(),
            Some('p'),
        );
        assert_eq!(
            Rucksack::new("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL").get_the_same_type_in_compartments(),
            Some('L'),
        );
        assert_eq!(
            Rucksack::new("PmmdzqPrVvPwwTWBwg").get_the_same_type_in_compartments(),
            Some('P'),
        );
        assert_eq!(
            Rucksack::new("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn").get_the_same_type_in_compartments(),
            Some('v'),
        );
        assert_eq!(
            Rucksack::new("ttgJtRGJQctTZtZT").get_the_same_type_in_compartments(),
            Some('t'),
        );
        assert_eq!(
            Rucksack::new("CrZsJsPPZsGzwwsLwLmpwMDw").get_the_same_type_in_compartments(),
            Some('s'),
        );
    }

    #[test]
    fn item_type_priority() {
        assert_eq!(get_item_priority('p'), 16);
        assert_eq!(get_item_priority('L'), 38);
        assert_eq!(get_item_priority('P'), 42);
        assert_eq!(get_item_priority('v'), 22);
        assert_eq!(get_item_priority('t'), 20);
        assert_eq!(get_item_priority('s'), 19);
    }

    #[test]
    fn test_part_1() {
        let result = solve_part_1("./resources/test_data.txt");
        assert_eq!(result, 157);
    }
}
