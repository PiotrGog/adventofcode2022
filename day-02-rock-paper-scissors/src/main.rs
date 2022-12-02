use std::{cmp::Ordering, fs};

fn load_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

mod part_1 {
    use super::*;

    pub fn parse_data(string_data: String) -> Vec<(Hand, Hand)> {
        let map_symbol_to_hand = |symbol: &str| match symbol {
            "A" | "X" => Hand::Rock,
            "B" | "Y" => Hand::Paper,
            "C" | "Z" => Hand::Scissors,
            _ => panic!("Unsupported value"),
        };

        string_data
            .lines()
            .map(|line| {
                let mut splitted = line.split_ascii_whitespace();
                let oponent = map_symbol_to_hand(splitted.next().unwrap());
                let my = map_symbol_to_hand(splitted.next().unwrap());
                (oponent, my)
            })
            .collect()
    }
}

mod part_2 {
    use super::*;

    pub fn parse_data(string_data: String) -> Vec<RoundResult> {
        let map_symbol_to_hand = |symbol: &str| match symbol {
            "A" => Hand::Rock,
            "B" => Hand::Paper,
            "C" => Hand::Scissors,
            _ => panic!("Unsupported value"),
        };

        let map_symbol_to_result = |symbol: &str, opponent: Hand| match symbol {
            "X" => RoundResult::Lose(opponent),
            "Y" => RoundResult::Draw(opponent),
            "Z" => RoundResult::Win(opponent),
            _ => panic!("Unsupported value"),
        };

        string_data
            .lines()
            .map(|line| {
                let mut splitted = line.split_ascii_whitespace();
                let oponent = map_symbol_to_hand(splitted.next().unwrap());
                map_symbol_to_result(splitted.next().unwrap(), oponent)
            })
            .collect()
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    pub fn get_score(&self) -> usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    pub fn play(&self, opponent: &Self) -> usize {
        (match self.cmp(opponent) {
            Ordering::Less => 0,
            Ordering::Equal => 3,
            Ordering::Greater => 6,
        }) as usize
            + self.get_score()
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Rock, Self::Scissors) => Ordering::Greater,
            (Self::Rock, Self::Paper) => Ordering::Less,
            (Self::Paper, Self::Rock) => Ordering::Greater,
            (Self::Paper, Self::Scissors) => Ordering::Less,
            (Self::Scissors, Self::Paper) => Ordering::Greater,
            (Self::Scissors, Self::Rock) => Ordering::Less,
            _ => Ordering::Equal,
        }
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone, Copy)]
pub enum RoundResult {
    Lose(Hand),
    Draw(Hand),
    Win(Hand),
}

impl RoundResult {
    pub fn get_expected_opponent_and_my_hand(&self) -> (Hand, Hand) {
        match self {
            Self::Lose(Hand::Paper) => (Hand::Paper, Hand::Rock),
            Self::Lose(Hand::Rock) => (Hand::Rock, Hand::Scissors),
            Self::Lose(Hand::Scissors) => (Hand::Scissors, Hand::Paper),
            Self::Win(Hand::Paper) => (Hand::Paper, Hand::Scissors),
            Self::Win(Hand::Rock) => (Hand::Rock, Hand::Paper),
            Self::Win(Hand::Scissors) => (Hand::Scissors, Hand::Rock),
            Self::Draw(hand) => (*hand, *hand),
        }
    }

    pub fn play(&self) -> usize {
        let (opponent, my) = self.get_expected_opponent_and_my_hand();
        my.play(&opponent)
    }
}

pub fn solve_part_1(file_path: &str) -> usize {
    let data = load_file(file_path);
    let rounds = part_1::parse_data(data);
    rounds
        .into_iter()
        .map(|(opponent, my)| my.play(&opponent))
        .sum()
}

pub fn solve_part_2(file_path: &str) -> usize {
    let data = load_file(file_path);
    let rounds = part_2::parse_data(data);
    rounds
        .into_iter()
        .map(|expected_result| expected_result.play())
        .sum()
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
    fn load_data_function_returns_two_strategies() {
        let data = load_file("./resources/test_data.txt");
        let result = part_1::parse_data(data);
        assert_eq!(
            result,
            vec![
                (Hand::Rock, Hand::Paper),
                (Hand::Paper, Hand::Rock),
                (Hand::Scissors, Hand::Scissors)
            ]
        );
    }

    #[test]
    fn scissors_greater_than_paper() {
        assert!(Hand::Scissors > Hand::Paper);
    }

    #[test]
    fn paper_greater_than_rock() {
        assert!(Hand::Paper > Hand::Rock);
    }

    #[test]
    fn rock_greater_than_scissors() {
        assert!(Hand::Rock > Hand::Scissors);
    }

    #[test]
    fn test_part_1() {
        let result = solve_part_1("./resources/test_data.txt");
        assert_eq!(result, 15);
    }

    #[test]
    fn test_part_2() {
        let result = solve_part_2("./resources/test_data.txt");
        assert_eq!(result, 12);
    }
}
