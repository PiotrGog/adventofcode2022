use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn load_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Position(isize, isize);

fn parse_data(data: String) -> HashSet<Position> {
    let mut elves = HashSet::new();
    for (row, line) in data.trim().lines().enumerate() {
        for (column, symbol) in line.chars().enumerate() {
            if symbol == '#' {
                elves.insert(Position(row as isize, column as isize));
            }
        }
    }
    elves
}

fn propose_positions(elves: HashSet<Position>, round_number: usize) -> HashMap<Position, Position> {
    let get_adjecement_for_proposed_north = |&position| {
        let Position(row, column) = position;
        HashSet::from([
            Position(row - 1, column - 1),
            Position(row - 1, column),
            Position(row - 1, column + 1),
        ])
    };
    let get_adjecement_for_proposed_south = |&position| {
        let Position(row, column) = position;
        HashSet::from([
            Position(row + 1, column - 1),
            Position(row + 1, column),
            Position(row + 1, column + 1),
        ])
    };
    let get_adjecement_for_proposed_west = |&position| {
        let Position(row, column) = position;
        HashSet::from([
            Position(row - 1, column - 1),
            Position(row, column - 1),
            Position(row + 1, column - 1),
        ])
    };
    let get_adjecement_for_proposed_east = |&position| {
        let Position(row, column) = position;
        HashSet::from([
            Position(row - 1, column + 1),
            Position(row, column + 1),
            Position(row + 1, column + 1),
        ])
    };

    elves
        .iter()
        .map(|position| {
            let adjacement_north = get_adjecement_for_proposed_north(position);
            let adjacement_south = get_adjecement_for_proposed_south(position);
            let adjacement_west = get_adjecement_for_proposed_west(position);
            let adjacement_east = get_adjecement_for_proposed_east(position);
            let empty_adjacement_north = adjacement_north
                .into_iter()
                .all(|position| !elves.contains(&position));
            let empty_adjacement_south = adjacement_south
                .into_iter()
                .all(|position| !elves.contains(&position));
            let empty_adjacement_west = adjacement_west
                .into_iter()
                .all(|position| !elves.contains(&position));
            let empty_adjacement_east = adjacement_east
                .into_iter()
                .all(|position| !elves.contains(&position));
            let Position(row, column) = position;
            if empty_adjacement_north
                && empty_adjacement_south
                && empty_adjacement_west
                && empty_adjacement_east
            {
                return (*position, *position);
            }
            let propositions = [
                (empty_adjacement_north, Position(row - 1, *column)),
                (empty_adjacement_south, Position(row + 1, *column)),
                (empty_adjacement_west, Position(*row, column - 1)),
                (empty_adjacement_east, Position(*row, column + 1)),
            ];
            for i in 0..4 {
                let (condition, result) = propositions[(i + round_number) % 4];
                if condition {
                    return (*position, result);
                }
            }
            (*position, *position)
        })
        .collect()
}

fn new_elves_position(proposed_position: HashMap<Position, Position>) -> HashSet<Position> {
    let counted_proposed_positions = proposed_position.clone().into_iter().fold(
        HashMap::new(),
        |mut counter, (_, proposed_position)| {
            counter
                .entry(proposed_position)
                .and_modify(|count| *count += 1)
                .or_insert(1);
            counter
        },
    );
    proposed_position
        .into_iter()
        .map(|(current_position, proposed_position)| {
            if counted_proposed_positions.get(&proposed_position).unwrap() == &1 {
                proposed_position
            } else {
                current_position
            }
        })
        .collect()
}

fn round(elves: HashSet<Position>, round_number: usize) -> HashSet<Position> {
    let proposed_positions = propose_positions(elves, round_number);
    new_elves_position(proposed_positions)
}

fn solve_part_1(file_path: &str) -> usize {
    let data = load_file(file_path);
    let elves = parse_data(data);
    let elves = (0..10).fold(elves, |elves, round_number| round(elves, round_number));
    let number_of_elves = elves.len() as isize;
    let (min_row, max_row, min_column, max_column) = elves.into_iter().fold(
        (isize::MAX, isize::MIN, isize::MAX, isize::MIN),
        |(min_row, max_row, min_column, max_column), position| {
            (
                min_row.min(position.0),
                max_row.max(position.0),
                min_column.min(position.1),
                max_column.max(position.1),
            )
        },
    );

    (((max_row - min_row + 1) * (max_column - min_column + 1)) - number_of_elves) as usize
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
        assert_eq!(result, 110);
    }
}
