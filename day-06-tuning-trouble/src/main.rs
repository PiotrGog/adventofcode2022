use std::{collections::HashSet, fs};

fn load_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

pub fn solve_part_1(data: String) -> Option<usize> {
    data.chars()
        .collect::<Vec<_>>()
        .windows(4)
        .enumerate()
        .find_map(|(idx, window)| {
            if HashSet::<char>::from_iter(window.to_owned()).len() == 4 {
                Some(idx + 4)
            } else {
                None
            }
        })
}

fn part_1(data: String) {
    let result = solve_part_1(data);
    println!("Part 1 result: {:?}", result);
}

fn main() {
    const FILE_PATH: &str = "./resources/puzzle.txt";
    let data = load_file(FILE_PATH);
    part_1(data.clone());
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
}
