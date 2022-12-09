use std::{collections::HashSet, fs, str::FromStr};

fn load_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

fn parse_data(data: String) -> Vec<Move> {
    data.trim()
        .lines()
        .map(|line| Move::from_str(line).unwrap())
        .collect::<Vec<_>>()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Move {
    Up(i64),
    Down(i64),
    Right(i64),
    Left(i64),
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splitted_data = s.trim().split_whitespace().collect::<Vec<_>>();
        let move_value = splitted_data[1].parse().unwrap();
        match splitted_data[0] {
            "U" => Ok(Self::Up(move_value)),
            "D" => Ok(Self::Down(move_value)),
            "R" => Ok(Self::Right(move_value)),
            "L" => Ok(Self::Left(move_value)),
            val => Err(format!("Unsupported direction {}", val)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Rope {
    knots: Vec<(i64, i64)>,
}

impl Rope {
    fn new(len: usize) -> Self {
        assert!(len >= 2);
        Self {
            knots: vec![(0, 0); len],
        }
    }

    fn move_u(&mut self) {
        let mut head = &mut self.knots[0];
        head.0 += 1;
        self.move_tail();
    }
    fn move_d(&mut self) {
        let mut head = &mut self.knots[0];
        head.0 -= 1;
        self.move_tail();
    }
    fn move_r(&mut self) {
        let mut head = &mut self.knots[0];
        head.1 += 1;
        self.move_tail();
    }
    fn move_l(&mut self) {
        let mut head = &mut self.knots[0];
        head.1 -= 1;
        self.move_tail();
    }

    fn move_tail(&mut self) {
        for knot in 1..self.knots.len() {
            let head = self.knots[knot - 1];
            let mut tail = self.knots[knot];

            let (shift_x, shift_y) = (head.0 - tail.0, head.1 - tail.1);
            assert!(shift_x <= 2 && shift_y <= 2, "{:?}", self.knots);

            if shift_x.abs() >= 2 && shift_y.abs() >= 2 {
                tail.0 += shift_x - shift_x.signum();
                tail.1 += shift_y - shift_y.signum();
            } else if shift_x.abs() > 1 {
                tail.0 += shift_x - shift_x.signum();
                tail.1 += shift_y;
            } else if shift_y.abs() > 1 {
                tail.0 += shift_x;
                tail.1 += shift_y - shift_y.signum();
            }
            self.knots[knot] = tail;
        }
    }
}

fn count_visited_fields_by_rope_tail(rope: Rope, movements: Vec<Move>) -> usize {
    let the_last_knot_index = rope.knots.len() - 1;
    movements
        .into_iter()
        .fold(
            (rope, HashSet::new()),
            |(mut rope, mut tail_positions), m| {
                match m {
                    Move::Up(x) => (0..x).into_iter().for_each(|_| {
                        rope.move_u();
                        tail_positions.insert(rope.knots[the_last_knot_index]);
                    }),
                    Move::Down(x) => (0..x).into_iter().for_each(|_| {
                        rope.move_d();
                        tail_positions.insert(rope.knots[the_last_knot_index]);
                    }),
                    Move::Right(x) => (0..x).into_iter().for_each(|_| {
                        rope.move_r();
                        tail_positions.insert(rope.knots[the_last_knot_index]);
                    }),
                    Move::Left(x) => (0..x).into_iter().for_each(|_| {
                        rope.move_l();
                        tail_positions.insert(rope.knots[the_last_knot_index]);
                    }),
                };
                (rope, tail_positions)
            },
        )
        .1
        .len()
}

fn solve_part_1(file_path: &str) -> usize {
    let data = load_file(file_path);
    let movements = parse_data(data);
    count_visited_fields_by_rope_tail(Rope::new(2), movements)
}

fn solve_part_2(file_path: &str) -> usize {
    let data = load_file(file_path);
    let movements = parse_data(data);
    count_visited_fields_by_rope_tail(Rope::new(10), movements)
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
        assert_eq!(result, 13);
    }

    #[test]
    fn test_part_2() {
        let result = solve_part_2("./resources/test_data.txt");
        assert_eq!(result, 1);
    }

    #[test]
    fn test_parse_data() {
        let data = load_file("./resources/test_data.txt");
        let moves = parse_data(data);
        assert_eq!(
            moves,
            vec![
                Move::Right(4),
                Move::Up(4),
                Move::Left(3),
                Move::Down(1),
                Move::Right(4),
                Move::Down(1),
                Move::Left(5),
                Move::Right(2),
            ]
        );
    }

    #[test]
    fn update_tail_1() {
        let mut rope = Rope {
            knots: vec![(1, 2), (1, 0)],
        };
        rope.move_tail();
        assert_eq!(
            rope,
            Rope {
                knots: vec![(1, 2), (1, 1)]
            }
        );
    }

    #[test]
    fn update_tail_2() {
        let mut rope = Rope {
            knots: vec![(3, 0), (1, 0)],
        };
        rope.move_tail();
        assert_eq!(
            rope,
            Rope {
                knots: vec![(3, 0), (2, 0)]
            }
        );
    }

    #[test]
    fn update_tail_3() {
        let mut rope = Rope {
            knots: vec![(1, 0), (3, 0)],
        };
        rope.move_tail();
        assert_eq!(
            rope,
            Rope {
                knots: vec![(1, 0), (2, 0)]
            }
        );
    }

    #[test]
    fn update_tail_4() {
        let mut rope = Rope {
            knots: vec![(1, 0), (2, 1)],
        };
        rope.move_tail();
        assert_eq!(
            rope,
            Rope {
                knots: vec![(1, 0), (2, 1)]
            }
        );
    }

    #[test]
    fn update_tail_5() {
        let mut rope = Rope {
            knots: vec![(3, 1), (1, 0)],
        };
        rope.move_tail();
        assert_eq!(
            rope,
            Rope {
                knots: vec![(3, 1), (2, 1)]
            }
        );
    }
}
