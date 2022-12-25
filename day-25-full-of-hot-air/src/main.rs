use std::{collections::VecDeque, fs};

fn load_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

fn parse_data(data: String) -> Vec<String> {
    data.trim().lines().map(String::from).collect()
}

fn snafu_to_decimal(snafu: &str) -> u64 {
    let length = snafu.len() as u32;
    snafu
        .chars()
        .enumerate()
        .fold(0_i64, |dec, (i, symbol)| match symbol {
            '-' => dec + 5_i64.pow(length - i as u32 - 1) * (-1),
            '=' => dec + 5_i64.pow(length - i as u32 - 1) * (-2),
            num if num.is_digit(3) => {
                dec + 5_i64.pow(length - i as u32 - 1) * num.to_digit(10).unwrap() as i64
            }
            other => panic!("Can't convert SNAFU number {snafu} to dec (incorrect symbol {other})"),
        }) as u64
}

fn decimal_to_snafu(mut dec: u64) -> String {
    let snafu_number_base = 5;

    let mut result = VecDeque::new();
    while dec > 0 {
        let rem = dec % snafu_number_base;
        result.push_front(rem as i64);
        dec /= snafu_number_base;
    }

    let mut result: VecDeque<_> = result.into_iter().rev().collect();

    let mut i = 0;
    while i < result.len() {
        if result[i] == 4 || result[i] == 3 {
            if i + 1 < result.len() {
                result[i + 1] += 1
            } else {
                result.push_back(1)
            }
            result[i] = result[i] - snafu_number_base as i64;
        } else if result[i] % snafu_number_base as i64 == 0 {
            if i + 1 < result.len() {
                result[i + 1] += result[i] / snafu_number_base as i64
            } else {
                result.push_back(result[i] / snafu_number_base as i64)
            }
            result[i] = 0;
        }
        i += 1;
    }

    result
        .into_iter()
        .rev()
        .map(|num| {
            char::from_digit(num as u32, 10).unwrap_or_else(|| {
                if num == -1 {
                    '-'
                } else if num == -2 {
                    '='
                } else {
                    panic!()
                }
            })
        })
        .collect()
}

fn solve_part_1(file_path: &str) -> String {
    let data = load_file(file_path);
    let numbers = parse_data(data);
    decimal_to_snafu(numbers.into_iter().map(|num| snafu_to_decimal(&num)).sum())
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
        assert_eq!(&result, "2=-1=0");
    }

    #[test]
    fn snafu_to_decimal_1() {
        assert_eq!(snafu_to_decimal("1=-0-2"), 1747);
    }

    #[test]
    fn snafu_to_decimal_2() {
        assert_eq!(snafu_to_decimal("12111"), 906)
    }
    #[test]
    fn snafu_to_decimal_3() {
        assert_eq!(snafu_to_decimal("2=0="), 198)
    }
    #[test]
    fn snafu_to_decimal_4() {
        assert_eq!(snafu_to_decimal("21"), 11)
    }
    #[test]
    fn snafu_to_decimal_5() {
        assert_eq!(snafu_to_decimal("2=01"), 201)
    }
    #[test]
    fn snafu_to_decimal_6() {
        assert_eq!(snafu_to_decimal("111"), 31)
    }
    #[test]
    fn snafu_to_decimal_7() {
        assert_eq!(snafu_to_decimal("20012"), 1257);
    }
    #[test]
    fn snafu_to_decimal_8() {
        assert_eq!(snafu_to_decimal("112"), 32)
    }
    #[test]
    fn snafu_to_decimal_9() {
        assert_eq!(snafu_to_decimal("1=-1="), 353)
    }
    #[test]
    fn snafu_to_decimal_10() {
        assert_eq!(snafu_to_decimal("1-12"), 107)
    }
    #[test]
    fn snafu_to_decimal_11() {
        assert_eq!(snafu_to_decimal("12"), 7)
    }
    #[test]
    fn snafu_to_decimal_12() {
        assert_eq!(snafu_to_decimal("1="), 3)
    }
    #[test]
    fn snafu_to_decimal_13() {
        assert_eq!(snafu_to_decimal("122"), 37)
    }

    #[test]
    fn decimal_to_snafu_1() {
        assert_eq!(&decimal_to_snafu(1), "1")
    }
    #[test]
    fn decimal_to_snafu_2() {
        assert_eq!(&decimal_to_snafu(2), "2")
    }
    #[test]
    fn decimal_to_snafu_3() {
        assert_eq!(&decimal_to_snafu(3), "1=")
    }
    #[test]
    fn decimal_to_snafu_4() {
        assert_eq!(&decimal_to_snafu(4), "1-")
    }
    #[test]
    fn decimal_to_snafu_5() {
        assert_eq!(&decimal_to_snafu(5), "10")
    }
    #[test]
    fn decimal_to_snafu_6() {
        assert_eq!(&decimal_to_snafu(6), "11")
    }
    #[test]
    fn decimal_to_snafu_7() {
        assert_eq!(&decimal_to_snafu(7), "12")
    }
    #[test]
    fn decimal_to_snafu_8() {
        assert_eq!(&decimal_to_snafu(8), "2=")
    }
    #[test]
    fn decimal_to_snafu_9() {
        assert_eq!(&decimal_to_snafu(9), "2-")
    }
    #[test]
    fn decimal_to_snafu_10() {
        assert_eq!(&decimal_to_snafu(10), "20")
    }
    #[test]
    fn decimal_to_snafu_11() {
        assert_eq!(&decimal_to_snafu(15), "1=0")
    }
    #[test]
    fn decimal_to_snafu_12() {
        assert_eq!(&decimal_to_snafu(20), "1-0")
    }
    #[test]
    fn decimal_to_snafu_13() {
        assert_eq!(&decimal_to_snafu(2022), "1=11-2")
    }
    #[test]
    fn decimal_to_snafu_14() {
        assert_eq!(&decimal_to_snafu(12345), "1-0---0")
    }
    #[test]
    fn decimal_to_snafu_15() {
        assert_eq!(&decimal_to_snafu(314159265), "1121-1110-1=0")
    }
}
