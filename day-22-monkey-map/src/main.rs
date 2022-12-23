use std::{collections::HashSet, fs};

fn load_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

fn parse_data(data: String) -> (Map, Path) {
    let map_and_path_strings: Vec<_> = data.split("\n\n").collect();

    let mut area = vec![];
    for (row, line) in map_and_path_strings[0].lines().enumerate() {
        for (column, symbol) in line.chars().enumerate() {
            if !symbol.is_whitespace() {
                area.push((row, column, symbol));
            }
        }
    }

    let walls = area
        .iter()
        .filter_map(|(row, column, symbol)| {
            if symbol == &'#' {
                Some((*row as i64, *column as i64))
            } else {
                None
            }
        })
        .collect();

    let area = area
        .into_iter()
        .map(|(row, column, _)| (row as i64, column as i64))
        .collect();

    let string_path_instructions = map_and_path_strings[1]
        .replace("R", " R ")
        .replace("L", " L ")
        .trim()
        .to_string();

    let instructions = string_path_instructions.split_ascii_whitespace().fold(
        vec![],
        |mut instructions_acc, token| {
            instructions_acc.push(match token {
                "L" => PathInstruction::Rotate('L'),
                "R" => PathInstruction::Rotate('R'),
                value => PathInstruction::Move(value.parse().unwrap()),
            });
            instructions_acc
        },
    );

    (Map { area, walls }, Path { instructions })
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Map {
    area: HashSet<(i64, i64)>,
    walls: HashSet<(i64, i64)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum PathInstruction {
    Move(i64),
    Rotate(char),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Path {
    instructions: Vec<PathInstruction>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct PositionAndOrientation {
    position: (i64, i64),
    orientation: Orientation,
}

impl PositionAndOrientation {
    fn rotate(&mut self, rotate: char) {
        self.orientation.rotate(rotate);
    }

    fn go(&mut self, distance: i64, map: &Map) {
        self.position = match self.orientation {
            Orientation::East => {
                let (first, last) = Self::get_first_and_last_columns_in_row(self.position.0, map);
                (0..=distance)
                    .map(|i| {
                        (
                            self.position.0,
                            (self.position.1 + i - first).rem_euclid(last + 1 - first) + first,
                        )
                    })
                    .take_while(|position| !map.walls.contains(position))
                    .last()
                    .unwrap()
            }
            Orientation::West => {
                let (first, last) = Self::get_first_and_last_columns_in_row(self.position.0, map);
                (0..=distance)
                    .map(|i| {
                        (
                            self.position.0,
                            (self.position.1 - i - first).rem_euclid(last + 1 - first) + first,
                        )
                    })
                    .take_while(|position| !map.walls.contains(position))
                    .last()
                    .unwrap()
            }
            Orientation::South => {
                let (first, last) = Self::get_first_and_last_rows_in_column(self.position.1, map);
                (0..=distance)
                    .map(|i| {
                        (
                            (self.position.0 + i - first).rem_euclid(last + 1 - first) + first,
                            self.position.1,
                        )
                    })
                    .take_while(|position| !map.walls.contains(position))
                    .last()
                    .unwrap()
            }
            Orientation::North => {
                let (first, last) = Self::get_first_and_last_rows_in_column(self.position.1, map);
                (0..=distance)
                    .map(|i| {
                        (
                            (self.position.0 - i - first).rem_euclid(last + 1 - first) + first,
                            self.position.1,
                        )
                    })
                    .take_while(|position| !map.walls.contains(position))
                    .last()
                    .unwrap()
            }
        }
    }

    fn get_first_and_last_columns_in_row(row: i64, map: &Map) -> (i64, i64) {
        let mut rows: Vec<_> = map
            .area
            .iter()
            .filter_map(|position| {
                if position.0 == row {
                    Some(position.1)
                } else {
                    None
                }
            })
            .collect();
        rows.sort();
        (*rows.first().unwrap(), *rows.last().unwrap())
    }

    fn get_first_and_last_rows_in_column(column: i64, map: &Map) -> (i64, i64) {
        let mut columns: Vec<_> = map
            .area
            .iter()
            .filter_map(|position| {
                if position.1 == column {
                    Some(position.0)
                } else {
                    None
                }
            })
            .collect();
        columns.sort();
        (*columns.first().unwrap(), *columns.last().unwrap())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Orientation {
    North,
    East,
    South,
    West,
}

impl Orientation {
    fn rotate(&mut self, rotate: char) {
        *self = match (&self, rotate) {
            (Self::North, 'L') => Self::West,
            (Self::East, 'L') => Self::North,
            (Self::South, 'L') => Self::East,
            (Self::West, 'L') => Self::South,
            (Self::North, 'R') => Self::East,
            (Self::East, 'R') => Self::South,
            (Self::South, 'R') => Self::West,
            (Self::West, 'R') => Self::North,
            _ => panic!(),
        };
    }
}

fn solve_part_1(file_path: &str) -> i64 {
    let data = load_file(file_path);
    let (map, path) = parse_data(data);
    let start_position = map
        .area
        .iter()
        .filter(|position| position.0 == 0 && !map.walls.contains(position))
        .min_by_key(|position| position.1)
        .unwrap();

    let mut me = PositionAndOrientation {
        position: *start_position,
        orientation: Orientation::East,
    };

    path.instructions
        .into_iter()
        .for_each(|instruction| match instruction {
            PathInstruction::Move(x) => me.go(x, &map),
            PathInstruction::Rotate(x) => me.rotate(x),
        });

    1000 * (me.position.0 + 1)
        + 4 * (me.position.1 + 1)
        + match me.orientation {
            Orientation::East => 0,
            Orientation::South => 1,
            Orientation::West => 2,
            Orientation::North => 3,
        }
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
        assert_eq!(result, 6032);
    }
}
