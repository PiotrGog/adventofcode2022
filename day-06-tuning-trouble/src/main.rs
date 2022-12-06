use std::{collections::HashSet, fs};

fn load_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

pub fn find_distinct_characters_idx(data: String, window_width: usize) -> Option<usize> {
    data.chars()
        .collect::<Vec<_>>()
        .windows(window_width)
        .enumerate()
        .find_map(|(idx, window)| {
            if HashSet::<char>::from_iter(window.to_owned()).len() == window_width {
                Some(idx + window_width)
            } else {
                None
            }
        })
}

pub fn solve_part_1(data: String) -> Option<usize> {
    find_distinct_characters_idx(data, 4)
}

pub fn solve_part_2(data: String) -> Option<usize> {
    find_distinct_characters_idx(data, 14)
}

fn part_1(data: String) {
    let result = solve_part_1(data);
    println!("Part 1 result: {:?}", result);
}

fn part_2(data: String) {
    let result = solve_part_2(data);
    println!("Part 2 result: {:?}", result);
}

fn main() {
    const FILE_PATH: &str = "./resources/puzzle.txt";
    let data = load_file(FILE_PATH);
    part_1(data.clone());
    part_2(data.clone());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_data_1() {
        let result = solve_part_1("mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string());
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_1_data_2() {
        let result = solve_part_1("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string());
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_1_data_3() {
        let result = solve_part_1("nppdvjthqldpwncqszvftbrmjlhg".to_string());
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_1_data_4() {
        let result = solve_part_1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string());
        assert_eq!(result, Some(10));
    }

    #[test]
    fn test_part_1_data_5() {
        let result = solve_part_1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string());
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_2_data_1() {
        let result = solve_part_2("mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string());
        assert_eq!(result, Some(19));
    }

    #[test]
    fn test_part_2_data_2() {
        let result = solve_part_2("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string());
        assert_eq!(result, Some(23));
    }

    #[test]
    fn test_part_2_data_3() {
        let result = solve_part_2("nppdvjthqldpwncqszvftbrmjlhg".to_string());
        assert_eq!(result, Some(23));
    }

    #[test]
    fn test_part_2_data_4() {
        let result = solve_part_2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string());
        assert_eq!(result, Some(29));
    }

    #[test]
    fn test_part_2_data_5() {
        let result = solve_part_2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string());
        assert_eq!(result, Some(26));
    }
}
