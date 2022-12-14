use std::{collections::HashSet, fs};
fn load_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

fn parse_data(data: String) -> HashSet<Position> {
    data.trim()
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|point| {
                    let mut coordinates = point.split(',');
                    (
                        coordinates.next().unwrap().parse().unwrap(),
                        coordinates.next().unwrap().parse().unwrap(),
                    )
                })
                .collect::<Vec<(usize, usize)>>()
                .windows(2)
                .fold(HashSet::new(), |mut single_line_container, window| {
                    let line_start = window[0];
                    let line_end = window[1];
                    let (x_line_start, y_line_start) = line_start;
                    let (x_line_end, y_line_end) = line_end;
                    for x in x_line_start.min(x_line_end)..=x_line_start.max(x_line_end) {
                        for y in y_line_start.min(y_line_end)..=y_line_start.max(y_line_end) {
                            single_line_container.insert(Position(x, y));
                        }
                    }
                    single_line_container
                })
        })
        .fold(HashSet::new(), |mut rocks_container, rocks| {
            rocks_container.extend(rocks.into_iter());
            rocks_container
        })
}

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
struct Position(usize, usize);

fn drop_sand(mut sand: Position, cave: &mut HashSet<Position>) -> bool {
    while let Some(next_sand) = drop_sand_iteration(sand, cave) {
        if next_sand == sand {
            cave.insert(next_sand);
            return true;
        } else {
            sand = next_sand;
        }
    }

    false
}

fn drop_sand_iteration(sand: Position, cave: &HashSet<Position>) -> Option<Position> {
    let top_element = cave
        .iter()
        .filter(|cave_element| sand.0 == cave_element.0 && sand.1 < cave_element.1)
        .min_by_key(|cave_element| cave_element.1);

    let top_element = top_element?;

    Some(
        if !cave.contains(&Position(top_element.0 - 1, top_element.1)) {
            Position(top_element.0 - 1, top_element.1)
        } else if !cave.contains(&Position(top_element.0 + 1, top_element.1)) {
            Position(top_element.0 + 1, top_element.1)
        } else {
            Position(top_element.0, top_element.1 - 1)
        },
    )
}

fn solve_part_1(file_path: &str) -> usize {
    let data = load_file(file_path);
    let mut cave = parse_data(data);
    (0..)
        .take_while(|_| drop_sand(Position(500, 0), &mut cave))
        .count()
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
        assert_eq!(result, 24);
    }

    #[test]
    fn test_parse_data() {
        let data = load_file("./resources/test_data.txt");
        let data = parse_data(data);
        assert_eq!(
            data,
            HashSet::from([
                Position(498, 4),
                Position(498, 5),
                Position(498, 6),
                Position(497, 6),
                Position(496, 6),
                Position(503, 4),
                Position(502, 5),
                Position(502, 4),
                Position(502, 6),
                Position(502, 7),
                Position(502, 8),
                Position(502, 9),
                Position(501, 9),
                Position(500, 9),
                Position(499, 9),
                Position(498, 9),
                Position(497, 9),
                Position(496, 9),
                Position(495, 9),
                Position(494, 9),
            ])
        );
    }
}
