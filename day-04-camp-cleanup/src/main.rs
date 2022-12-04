use std::fs;

fn load_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

fn parse_data(string_data: String) -> Vec<ElvesPair> {
    string_data
        .trim()
        .lines()
        .map(|line| {
            let splitted = line.split(',').collect::<Vec<_>>();
            let elf1 = splitted[0].split('-').collect::<Vec<_>>();
            let elf2 = splitted[1].split('-').collect::<Vec<_>>();
            ElvesPair(
                Range(elf1[0].parse().unwrap(), elf1[1].parse().unwrap()),
                Range(elf2[0].parse().unwrap(), elf2[1].parse().unwrap()),
            )
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Range(usize, usize);

impl Range {
    pub fn in_other(&self, other: &Self) -> bool {
        self.0 >= other.0 && self.1 <= other.1
    }

    pub fn overlap(&self, other: &Self) -> bool {
        (self.0 >= other.0 && self.0 <= other.1)
            || (self.1 >= other.0 && self.1 <= other.1)
            || (other.0 >= self.0 && other.0 <= self.1)
            || (other.1 >= self.0 && other.1 <= self.1)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct ElvesPair(Range, Range);

impl ElvesPair {
    pub fn can_reduce(&self) -> bool {
        self.0.in_other(&self.1) || self.1.in_other(&self.0)
    }

    pub fn overlap(&self) -> bool {
        self.0.overlap(&self.1)
    }
}

pub fn solve_part_1(file_path: &str) -> usize {
    let data = load_file(file_path);
    let elves_pair = parse_data(data);
    elves_pair.into_iter().filter(ElvesPair::can_reduce).count()
}

pub fn solve_part_2(file_path: &str) -> usize {
    let data = load_file(file_path);
    let elves_pair = parse_data(data);
    elves_pair.into_iter().filter(ElvesPair::overlap).count()
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
    fn load_data_function_returns_rucksacks() {
        let data = load_file("./resources/test_data.txt");
        let result = parse_data(data);
        assert_eq!(
            result,
            vec![
                ElvesPair(Range(2, 4), Range(6, 8)),
                ElvesPair(Range(2, 3), Range(4, 5)),
                ElvesPair(Range(5, 7), Range(7, 9)),
                ElvesPair(Range(2, 8), Range(3, 7)),
                ElvesPair(Range(6, 6), Range(4, 6)),
                ElvesPair(Range(2, 6), Range(4, 8)),
            ]
        );
    }

    #[test]
    fn test_part_1() {
        let result = solve_part_1("./resources/test_data.txt");
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part_2() {
        let result = solve_part_2("./resources/test_data.txt");
        assert_eq!(result, 4);
    }
}
