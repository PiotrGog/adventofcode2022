use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

use derivative::Derivative;

fn load_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

fn parse_data(data: String) -> Blizzards {
    let mut blizzards = HashSet::new();
    let mut rows = 0;
    let mut columns = 0;
    for (row, line) in data.trim().lines().enumerate() {
        for (column, symbol) in line.chars().enumerate() {
            let position = Position {
                row: row as i32 - 1,
                column: column as i32 - 1,
            };
            match symbol {
                '^' => blizzards.insert(Blizzard {
                    position,
                    direction: MoveDirection::Up,
                }),
                '>' => blizzards.insert(Blizzard {
                    position,
                    direction: MoveDirection::Right,
                }),
                'v' => blizzards.insert(Blizzard {
                    position,
                    direction: MoveDirection::Down,
                }),
                '<' => blizzards.insert(Blizzard {
                    position,
                    direction: MoveDirection::Left,
                }),
                '.' => true,
                '#' => {
                    rows = rows.max(position.row);
                    columns = columns.max(position.column);

                    true
                }
                other => panic!("Unexpected other token {other}"),
            };
        }
    }
    Blizzards {
        blizzards,
        rows,
        columns,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum MoveDirection {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Derivative)]
#[derivative(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Blizzard {
    position: Position,

    #[derivative(PartialEq = "ignore")]
    #[derivative(Hash = "ignore")]
    direction: MoveDirection,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Blizzards {
    blizzards: HashSet<Blizzard>,
    rows: i32,
    columns: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Position {
    row: i32,
    column: i32,
}

fn get_start_position(_blizzards: &Blizzards) -> Position {
    Position { row: -1, column: 0 }
}

fn get_stop_position(blizzards: &Blizzards) -> Position {
    Position {
        row: blizzards.rows,
        column: blizzards.columns - 1,
    }
}

type BlizzardsPositions = HashSet<Position>;

fn get_blizzards_positions_at(blizzards: &Blizzards, step: i32) -> BlizzardsPositions {
    let Blizzards {
        blizzards,
        rows,
        columns,
    } = blizzards;
    blizzards
        .iter()
        .map(|blizzard| match blizzard.direction {
            MoveDirection::Up => Position {
                row: (blizzard.position.row - step).rem_euclid(*rows),
                column: blizzard.position.column,
            },
            MoveDirection::Down => Position {
                row: (blizzard.position.row + step).rem_euclid(*rows),
                column: blizzard.position.column,
            },
            MoveDirection::Left => Position {
                row: blizzard.position.row,
                column: (blizzard.position.column - step).rem_euclid(*columns),
            },
            MoveDirection::Right => Position {
                row: blizzard.position.row,
                column: (blizzard.position.column + step).rem_euclid(*columns),
            },
        })
        .collect()
}

fn get_available_positions(
    current_position: &Position,
    blizzards: &Blizzards,
    blizzards_positions: &BlizzardsPositions,
) -> Vec<Position> {
    let Position { row, column } = current_position;
    let up = Position {
        row: row - 1,
        column: *column,
    };
    let down = Position {
        row: row + 1,
        column: *column,
    };
    let right = Position {
        row: *row,
        column: column + 1,
    };
    let left = Position {
        row: *row,
        column: column - 1,
    };

    let Blizzards {
        blizzards: _,
        rows,
        columns,
    } = blizzards;

    [
        if up.row >= 0 && up.column >= 0 && !blizzards_positions.contains(&up)
            || up == get_start_position(blizzards)
        {
            Some(up)
        } else {
            None
        },
        if down.row < *rows && down.column >= 0 && !blizzards_positions.contains(&down)
            || down == get_stop_position(blizzards)
        {
            Some(down)
        } else {
            None
        },
        if left.column >= 0 && left.row >= 0 && !blizzards_positions.contains(&left) {
            Some(left)
        } else {
            None
        },
        if right.column < *columns && right.row >= 0 && !blizzards_positions.contains(&right) {
            Some(right)
        } else {
            None
        },
        if !blizzards_positions.contains(current_position) {
            Some(*current_position)
        } else {
            None
        },
    ]
    .iter()
    .flatten()
    .cloned()
    .collect()
}

fn find_shortest_path(
    blizzards: &Blizzards,
    start_position: Position,
    target_position: Position,
) -> Option<i32> {
    let mut visited = HashSet::from([(start_position, 0)]);
    let mut queue = VecDeque::from([(start_position, 0)]);
    let mut precomputed_blizzards_positions = HashMap::new();

    let number_of_possible_configuration = nww(blizzards.rows, blizzards.columns);

    while let Some((current_position, step)) = queue.pop_front() {
        if current_position == target_position {
            return Some(step);
        }
        let next_step_num = step + 1;

        let blizzards_positions = precomputed_blizzards_positions
            .entry(next_step_num.rem_euclid(number_of_possible_configuration))
            .or_insert(get_blizzards_positions_at(blizzards, next_step_num));
        get_available_positions(&current_position, blizzards, blizzards_positions)
            .into_iter()
            .for_each(|position| {
                if !visited.contains(&(position, next_step_num)) {
                    queue.push_back((position, next_step_num));
                    visited.insert((position, next_step_num));
                }
            });
    }

    None
}

fn nwd(mut a: i32, mut b: i32) -> i32 {
    while a != b {
        if a > b {
            a -= b;
        } else {
            b -= a;
        }
    }
    a
}

fn nww(a: i32, b: i32) -> i32 {
    (a * b) / nwd(a, b)
}

fn solve_part_1(file_path: &str) -> Option<i32> {
    let data = load_file(file_path);
    let blizzards = parse_data(data);
    find_shortest_path(
        &blizzards,
        get_start_position(&blizzards),
        get_stop_position(&blizzards),
    )
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
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_load_and_parse_data() {
        let loaded_data = load_file("./resources/test_data.txt");
        let blizzards = parse_data(loaded_data);
        assert_eq!(blizzards.rows, 4);
        assert_eq!(blizzards.columns, 6);
    }

    #[test]
    fn test_get_start_position() {
        let loaded_data = load_file("./resources/test_data.txt");
        let blizzards = parse_data(loaded_data);
        assert_eq!(
            get_start_position(&blizzards),
            Position { row: -1, column: 0 }
        );
    }

    #[test]
    fn test_get_stop_position() {
        let loaded_data = load_file("./resources/test_data.txt");
        let blizzards = parse_data(loaded_data);
        assert_eq!(
            get_stop_position(&blizzards),
            Position { row: 4, column: 5 }
        );
    }
}
