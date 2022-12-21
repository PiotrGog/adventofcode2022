use std::fs;

fn load_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

fn parse_data(data: String) -> Vec<(usize, isize)> {
    data.trim()
        .lines()
        .enumerate()
        .map(|(pos, line)| (pos, line.parse().unwrap()))
        .collect()
}

fn mix(mut numbers: Vec<(usize, isize)>) -> Vec<(usize, isize)> {
    let size = numbers.len() as isize - 1;
    (0..numbers.len()).for_each(|next_number_position| {
        let index = numbers
            .iter()
            .position(|(position, _)| position == &next_number_position)
            .unwrap();
        let number = numbers.remove(index);
        let unwrapped_target_index = index as isize + number.1;
        let wrapped_target_index = unwrapped_target_index.rem_euclid(size) as usize;
        numbers.insert(wrapped_target_index, number);
    });

    numbers
}

fn solve_part_1(file_path: &str) -> isize {
    let data = load_file(file_path);
    let numbers = parse_data(data);
    let mixed_number = mix(numbers);
    let index = mixed_number
        .iter()
        .position(|(_, value)| value == &0)
        .unwrap();
    mixed_number[(1000 + index) % mixed_number.len()].1
        + mixed_number[(2000 + index) % mixed_number.len()].1
        + mixed_number[(3000 + index) % mixed_number.len()].1
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
        assert_eq!(result, 3);
    }
}
