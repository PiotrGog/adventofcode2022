use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

fn load_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

fn parse_data(data: String) -> (Area, Position, Position) {
    let mut start_position = None;
    let mut target_position = None;
    let area = data
        .trim()
        .lines()
        .enumerate()
        .map(|(line_number, line)| {
            line.chars()
                .enumerate()
                .map(|(character_number, c)| match c {
                    'a'..='z' => c,
                    'S' => {
                        start_position = Some(Position(line_number, character_number));
                        'a'
                    }
                    'E' => {
                        target_position = Some(Position(line_number, character_number));
                        'z'
                    }
                    other => panic!("Unexpected value {other}"),
                } as usize - 'a' as usize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let x_size = area.len();
    let y_size = area[0].len();
    (
        Area {
            area,
            x_size,
            y_size,
        },
        start_position.unwrap(),
        target_position.unwrap(),
    )
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Area {
    area: Vec<Vec<usize>>,
    x_size: usize,
    y_size: usize,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
struct Position(usize, usize);

fn get_available_positions_max_one_up(current_position: &Position, area: &Area) -> Vec<Position> {
    let Position(x, y) = current_position;
    let Area {
        area,
        x_size,
        y_size,
    } = area;
    let current_high = area[*x][*y];
    let mut result = vec![];
    let check_height = |x_new: usize, y_new: usize| {
        let area: &Vec<Vec<usize>> = area;
        area[x_new][y_new] <= current_high || area[x_new][y_new].abs_diff(current_high) <= 1
    };

    if *x > 0 && check_height(*x - 1, *y) {
        result.push(Position(x - 1, *y));
    }
    if x + 1 < *x_size && check_height(*x + 1, *y) {
        result.push(Position(x + 1, *y));
    }
    if *y > 0 && check_height(*x, *y - 1) {
        result.push(Position(*x, y - 1));
    }
    if y + 1 < *y_size && check_height(*x, *y + 1) {
        result.push(Position(*x, y + 1));
    }
    result
}

fn find_shortest_path(
    area: &Area,
    start_position: Position,
    target_position: Position,
) -> Option<(usize, HashMap<Position, Position>)> {
    let mut previous = HashMap::new();

    let mut queue = VecDeque::from([(start_position, 0)]);
    let mut visited = HashSet::from([start_position]);

    while let Some((current_position, distance)) = queue.pop_front() {
        if current_position == target_position {
            return Some((distance, previous));
        }
        get_available_positions_max_one_up(&current_position, area)
            .into_iter()
            .for_each(|position| {
                if !visited.contains(&position) {
                    previous.insert(position, current_position);
                    queue.push_back((position, distance + 1));
                    visited.insert(position);
                }
            });
    }

    None
}

fn solve_part_1(file_path: &str) -> Option<usize> {
    let data = load_file(file_path);
    let (area, start_position, target_position) = parse_data(data);
    if let Some((distance, _)) = find_shortest_path(&area, start_position, target_position) {
        Some(distance)
    } else {
        None
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
        assert_eq!(result, Some(31));
    }
}
