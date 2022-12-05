use std::fs;

fn load_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

fn parse_data(string_data: String) -> (Vec<Stack<char>>, Vec<RearrangmentProdecudure>) {
    let mut lines = string_data.lines();
    let stacks = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            line.replace("    ", "[_] ")
                .replace("[", " ")
                .replace("]", " ")
                .replace(" ", "")
        })
        .fold(vec![], |mut stacks, line| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| c != &'_' && !c.is_numeric())
                .for_each(|(idx, c)| {
                    while stacks.len() <= idx {
                        stacks.push(Stack::new());
                    }
                    stacks[idx].push(c);
                });
            stacks
        })
        .into_iter()
        .map(|mut stack| {
            stack.reverse();
            stack
        })
        .collect();

    let rearrangment_procedures = lines
        .map(|line| {
            let procedure_data = line
                .replace("move ", "")
                .replace("from", "")
                .replace(" to", "");
            let mut procedure_data = procedure_data.split_whitespace();
            RearrangmentProdecudure::new(
                procedure_data.next().unwrap().parse::<usize>().unwrap(),
                procedure_data.next().unwrap().parse::<usize>().unwrap() - 1,
                procedure_data.next().unwrap().parse::<usize>().unwrap() - 1,
            )
        })
        .collect();

    (stacks, rearrangment_procedures)
}

type Stack<T> = Vec<T>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct RearrangmentProdecudure {
    how_many: usize,
    from: usize,
    to: usize,
}

impl RearrangmentProdecudure {
    fn new(how_many: usize, from: usize, to: usize) -> Self {
        Self { how_many, from, to }
    }
}

fn build_string_from_stacks_top(stacks: Vec<Stack<char>>) -> String {
    stacks
        .into_iter()
        .fold("".to_string(), |mut acc, mut stack| {
            if let Some(value) = stack.pop() {
                acc.push(value)
            }
            acc
        })
}

pub fn solve_part_1(file_path: &str) -> String {
    let data = load_file(file_path);
    let (mut stacks, rearrangment_procedures) = parse_data(data);
    rearrangment_procedures.into_iter().for_each(|procedure| {
        for _ in 0..procedure.how_many {
            if let Some(value) = stacks[procedure.from].pop() {
                stacks[procedure.to].push(value);
            }
        }
    });
    build_string_from_stacks_top(stacks)
}

pub fn solve_part_2(file_path: &str) -> String {
    let data = load_file(file_path);
    let (mut stacks, rearrangment_procedures) = parse_data(data);
    rearrangment_procedures.into_iter().for_each(|procedure| {
        let to_insert = {
            let from = &mut stacks[procedure.from];
            let split_index = from.len().checked_sub(procedure.how_many).unwrap_or(0);
            from.split_off(split_index)
        };
        let to = &mut stacks[procedure.to];
        to.extend(to_insert);
    });
    build_string_from_stacks_top(stacks)
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
    fn load_data_function_returns_stacks_and_rearragment_procedure() {
        let data = load_file("./resources/test_data.txt");
        let result = parse_data(data);
        assert_eq!(
            result,
            (
                vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']],
                vec![
                    RearrangmentProdecudure::new(1, 1, 0),
                    RearrangmentProdecudure::new(3, 0, 2),
                    RearrangmentProdecudure::new(2, 1, 0),
                    RearrangmentProdecudure::new(1, 0, 1),
                ]
            )
        );
    }

    #[test]
    fn test_part_1() {
        let result = solve_part_1("./resources/test_data.txt");
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn test_part_2() {
        let result = solve_part_2("./resources/test_data.txt");
        assert_eq!(result, "MCD");
    }
}
