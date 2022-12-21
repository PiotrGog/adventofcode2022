use std::{collections::HashMap, fs};

fn load_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

fn parse_data(data: String) -> HashMap<String, Monkey> {
    data.trim()
        .lines()
        .map(|line| {
            let cleaned_line = line.replace(":", "");
            let splitted_line = cleaned_line.split_ascii_whitespace().collect::<Vec<_>>();
            let monkey = match splitted_line.len() {
                2 => Monkey::Number(splitted_line[1].parse().unwrap()),
                4 => Monkey::Operation(Operation {
                    monkey_name_1: splitted_line[1].to_string(),
                    monkey_name_2: splitted_line[3].to_string(),
                    operation: splitted_line[2].parse().unwrap(),
                }),
                _ => panic!(),
            };
            (splitted_line[0].to_string(), monkey)
        })
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Monkey {
    Number(isize),
    Operation(Operation),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Operation {
    monkey_name_1: String,
    monkey_name_2: String,
    operation: char,
}

fn calculate_yelled_value(monkey_name: &str, monkeys: &HashMap<String, Monkey>) -> isize {
    match monkeys.get(monkey_name).unwrap() {
        Monkey::Number(number) => *number,
        Monkey::Operation(Operation {
            monkey_name_1,
            monkey_name_2,
            operation,
        }) => {
            let value_1 = calculate_yelled_value(monkey_name_1, monkeys);
            let value_2 = calculate_yelled_value(monkey_name_2, monkeys);
            match operation {
                '+' => value_1 + value_2,
                '-' => value_1 - value_2,
                '*' => value_1 * value_2,
                '/' => value_1 / value_2,
                unsupported_operation => panic!("Unsupported operation {unsupported_operation}"),
            }
        }
    }
}

fn solve_part_1(file_path: &str) -> isize {
    let data = load_file(file_path);
    let monkeys = parse_data(data);
    calculate_yelled_value("root", &monkeys)
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
        assert_eq!(result, 152);
    }
}
