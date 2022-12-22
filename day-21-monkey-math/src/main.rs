use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

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

fn get_precomputed_yell_values(
    monkey_name: &str,
    monkeys: &HashMap<String, Monkey>,
) -> HashMap<String, isize> {
    match monkeys.get(monkey_name).unwrap() {
        Monkey::Number(number) => HashMap::from([(monkey_name.to_string(), *number)]),
        Monkey::Operation(Operation {
            monkey_name_1,
            monkey_name_2,
            operation,
        }) => {
            let mut values = get_precomputed_yell_values(monkey_name_1, monkeys);
            values.extend(get_precomputed_yell_values(monkey_name_2, monkeys));
            values.insert(
                monkey_name.to_string(),
                match operation {
                    '+' => values.get(monkey_name_1).unwrap() + values.get(monkey_name_2).unwrap(),
                    '-' => values.get(monkey_name_1).unwrap() - values.get(monkey_name_2).unwrap(),
                    '*' => values.get(monkey_name_1).unwrap() * values.get(monkey_name_2).unwrap(),
                    '/' => values.get(monkey_name_1).unwrap() / values.get(monkey_name_2).unwrap(),
                    unsupported_operation => {
                        panic!("Unsupported operation {unsupported_operation}")
                    }
                },
            );
            values
        }
    }
}

fn yells_depend_on(
    start_monkey_name: &str,
    dependency: &str,
    monkeys: &HashMap<String, Monkey>,
) -> HashSet<String> {
    if start_monkey_name == dependency {
        return HashSet::from([start_monkey_name.to_string()]);
    }

    match monkeys.get(start_monkey_name).unwrap() {
        Monkey::Number(_) => HashSet::new(),
        Monkey::Operation(Operation {
            monkey_name_1,
            monkey_name_2,
            ..
        }) => {
            let mut dependent = yells_depend_on(monkey_name_1, dependency, monkeys);
            dependent.extend(yells_depend_on(monkey_name_2, dependency, monkeys));
            if !dependent.is_empty() {
                dependent.insert(start_monkey_name.to_string());
            }
            dependent
        }
    }
}

fn solve_equation(
    monkey_name_1: &str,
    monkey_name_2: &str,
    unknown_yell: &str,
    monkeys: &HashMap<String, Monkey>,
) -> isize {
    let precomputed_values: HashMap<_, _> = get_precomputed_yell_values(monkey_name_1, &monkeys)
        .into_iter()
        .chain(get_precomputed_yell_values(monkey_name_2, &monkeys).into_iter())
        .collect();
    let dependent_yells: HashSet<_> = yells_depend_on(monkey_name_1, unknown_yell, &monkeys)
        .into_iter()
        .chain(yells_depend_on(monkey_name_2, unknown_yell, &monkeys).into_iter())
        .collect();

    assert_ne!(
        dependent_yells.contains(monkey_name_1),
        dependent_yells.contains(monkey_name_2)
    );
    let (mut actual_result, to_queue) = if dependent_yells.contains(monkey_name_1) {
        (
            *precomputed_values.get(monkey_name_2).unwrap(),
            monkeys.get(monkey_name_1).unwrap(),
        )
    } else if dependent_yells.contains(monkey_name_2) {
        (
            *precomputed_values.get(monkey_name_1).unwrap(),
            monkeys.get(monkey_name_2).unwrap(),
        )
    } else {
        panic!("Neither {monkey_name_1} nor {monkey_name_2} depend on {unknown_yell}");
    };

    let mut queue = VecDeque::from([to_queue]);
    while let Some(monkey) = queue.pop_front() {
        match monkey {
            Monkey::Operation(Operation {
                monkey_name_1,
                monkey_name_2,
                operation,
            }) => {
                assert_ne!(
                    dependent_yells.contains(monkey_name_1),
                    dependent_yells.contains(monkey_name_2)
                );
                actual_result = if dependent_yells.contains(monkey_name_1) {
                    let next_monkey = monkeys.get(monkey_name_1).unwrap();
                    if let Monkey::Operation(_) = next_monkey {
                        queue.push_back(next_monkey);
                    }
                    match operation {
                        '+' => actual_result - precomputed_values.get(monkey_name_2).unwrap(),
                        '-' => actual_result + precomputed_values.get(monkey_name_2).unwrap(),
                        '*' => actual_result / precomputed_values.get(monkey_name_2).unwrap(),
                        '/' => actual_result * precomputed_values.get(monkey_name_2).unwrap(),
                        unsupported_operation => {
                            panic!("Unsupported operation {unsupported_operation}")
                        }
                    }
                } else {
                    let next_monkey = monkeys.get(monkey_name_2).unwrap();
                    if let Monkey::Operation(_) = next_monkey {
                        queue.push_back(next_monkey);
                    }
                    match operation {
                        '+' => actual_result - precomputed_values.get(monkey_name_1).unwrap(),
                        '-' => precomputed_values.get(monkey_name_1).unwrap() - actual_result,
                        '*' => actual_result / precomputed_values.get(monkey_name_1).unwrap(),
                        '/' => precomputed_values.get(monkey_name_1).unwrap() / actual_result,
                        unsupported_operation => {
                            panic!("Unsupported operation {unsupported_operation}")
                        }
                    }
                }
            }
            _ => panic!("Required Monkey::Operation"),
        }
    }
    actual_result
}

fn solve_part_1(file_path: &str) -> isize {
    let data = load_file(file_path);
    let monkeys = parse_data(data);
    calculate_yelled_value("root", &monkeys)
}

fn solve_part_2(file_path: &str) -> isize {
    let data = load_file(file_path);
    let monkeys = parse_data(data);
    let unknown_yell = "humn";

    let (monkey_name_1, monkey_name_2) = match monkeys.get("root").unwrap() {
        Monkey::Operation(Operation {
            monkey_name_1,
            monkey_name_2,
            ..
        }) => (monkey_name_1, monkey_name_2),
        _ => panic!("'root' has to be operation"),
    };

    solve_equation(monkey_name_1, monkey_name_2, unknown_yell, &monkeys)
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
    fn test_part_1() {
        let result = solve_part_1("./resources/test_data.txt");
        assert_eq!(result, 152);
    }

    #[test]
    fn test_part_2() {
        let result = solve_part_2("./resources/test_data.txt");
        assert_eq!(result, 301);
    }
}
