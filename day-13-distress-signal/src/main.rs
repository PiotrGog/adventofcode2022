use std::{fs, str::FromStr};

fn load_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

fn parse_data(data: String) -> Vec<PacketsPair> {
    data.trim()
        .split("\n\n")
        .map(|packets_pair_string| {
            let packets_array = packets_pair_string.lines().collect::<Vec<_>>();
            PacketsPair(
                packets_array[0].parse().unwrap(),
                packets_array[1].parse().unwrap(),
            )
        })
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum PacketValue {
    List(Vec<PacketValue>),
    Integer(u8),
}

impl FromStr for PacketValue {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = Self::split_tokens(s);
        let mut tokens_iter = tokens.into_iter();
        // consume first openning bracket
        tokens_iter.next();
        Ok(Self::parse_recurent(&mut tokens_iter))
    }
}

impl PacketValue {
    fn split_tokens(s: &str) -> Vec<String> {
        s.replace(',', " ")
            .replace('[', "[ ")
            .replace(']', " ]")
            .split_whitespace()
            .map(str::to_owned)
            .collect()
    }

    fn parse_recurent(tokens_iter: &mut impl Iterator<Item = String>) -> Self {
        let mut packet = vec![];
        while let Some(token) = tokens_iter.next() {
            match token.as_str() {
                "[" => packet.push(Self::parse_recurent(tokens_iter)),
                "]" => break,
                val => packet.push(PacketValue::Integer(val.parse().unwrap())),
            }
        }

        PacketValue::List(packet)
    }

    fn as_list(&self) -> PacketValue {
        match self {
            Self::Integer(..) => Self::List(vec![self.clone()]),
            Self::List(..) => self.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct PacketsPair(PacketValue, PacketValue);

impl PacketsPair {
    fn is_right_order(&self) -> bool {
        Self::is_right_order_recurent(&self.0, &self.1) == CheckStatus::RightOrder
    }

    fn is_right_order_recurent(
        first_packet: &PacketValue,
        second_packet: &PacketValue,
    ) -> CheckStatus {
        match (first_packet, second_packet) {
            (PacketValue::Integer(val1), PacketValue::Integer(val2)) => {
                Self::compare_integers(val1, val2)
            }
            (PacketValue::List(list1), PacketValue::List(list2)) => {
                Self::compare_lists(list1, list2)
            }
            (packet1, packet2) => {
                Self::is_right_order_recurent(&packet1.as_list(), &packet2.as_list())
            }
        }
    }

    fn compare_integers(val1: &u8, val2: &u8) -> CheckStatus {
        match val1.cmp(val2) {
            std::cmp::Ordering::Less => CheckStatus::RightOrder,
            std::cmp::Ordering::Greater => CheckStatus::WrongOrder,
            std::cmp::Ordering::Equal => CheckStatus::Continue,
        }
    }

    fn compare_lists(list1: &Vec<PacketValue>, list2: &Vec<PacketValue>) -> CheckStatus {
        let list1_length = list1.len();
        let list2_length = list2.len();
        for i in 0..list1_length.min(list2_length) {
            match Self::is_right_order_recurent(&list1[i], &list2[i]) {
                CheckStatus::Continue => continue,
                other => return other,
            }
        }

        match list1_length.cmp(&list2_length) {
            std::cmp::Ordering::Less => CheckStatus::RightOrder,
            std::cmp::Ordering::Greater => CheckStatus::WrongOrder,
            std::cmp::Ordering::Equal => CheckStatus::Continue,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum CheckStatus {
    RightOrder,
    WrongOrder,
    Continue,
}

fn solve_part_1(file_path: &str) -> usize {
    let data = load_file(file_path);
    let packets_pairs = parse_data(data);
    packets_pairs
        .into_iter()
        .zip(1..)
        .filter_map(|(packet_pair, index)| {
            if packet_pair.is_right_order() {
                Some(index)
            } else {
                None
            }
        })
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
    fn test_part_1() {
        let result = solve_part_1("./resources/test_data.txt");
        assert_eq!(result, 13);
    }

    #[test]
    fn test_splitting_tokens() {
        let tokens = PacketValue::split_tokens("[1,[2,[3,[4,[5,6,0]]]],8,9]");
        let expected = vec![
            "[", "1", "[", "2", "[", "3", "[", "4", "[", "5", "6", "0", "]", "]", "]", "]", "8",
            "9", "]",
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_value_parsing() {
        let result = PacketValue::from_str("[1,[2,[3,[4,[5,6,0]]]],8,9]");
        let expected = Ok(PacketValue::List(vec![
            PacketValue::Integer(1),
            PacketValue::List(vec![
                PacketValue::Integer(2),
                PacketValue::List(vec![
                    PacketValue::Integer(3),
                    PacketValue::List(vec![
                        PacketValue::Integer(4),
                        PacketValue::List(vec![
                            PacketValue::Integer(5),
                            PacketValue::Integer(6),
                            PacketValue::Integer(0),
                        ]),
                    ]),
                ]),
            ]),
            PacketValue::Integer(8),
            PacketValue::Integer(9),
        ]));
        assert_eq!(result, expected);
    }
}
