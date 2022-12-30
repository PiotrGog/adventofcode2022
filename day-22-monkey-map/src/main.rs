use std::{
    collections::{HashMap, HashSet},
    fs,
    ops::ControlFlow,
};

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
                Some(Position(*row as i64, *column as i64))
            } else {
                None
            }
        })
        .collect();

    let area = area
        .into_iter()
        .map(|(row, column, _)| Position(row as i64, column as i64))
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
    area: HashSet<Position>,
    walls: HashSet<Position>,
}

#[derive(Debug, Clone)]
struct Cube {
    /*
        [0]
     [4][1][5]
        [2]
        [3]
    */
    tiles: HashMap<Position3D, Position>,
}

impl Cube {
    fn from_map(map: &Map) -> Self {
        let mut cube = Self {
            tiles: HashMap::new(),
        };

        let faces = 6;
        let face_surface = map.area.len() / faces;
        let face_size = (face_surface as f64).sqrt() as i64;

        let start_position = map
            .area
            .iter()
            .filter(|position| position.0 == 0 && !map.walls.contains(position))
            .min_by_key(|position| position.1)
            .unwrap();
        let mut visited = HashSet::new();
        Self::create_cube(&mut cube, &map, &start_position, &mut visited, face_size);
        cube
    }

    fn create_cube(
        cube: &mut Cube,
        map: &Map,
        start_position: &Position,
        visited: &mut HashSet<Position>,
        face_size: i64,
    ) {
        if visited.insert(start_position.clone()) {
            let translate = face_size - 1;
            let Position(start_x, start_y) = start_position;
            for x in 0..face_size {
                for y in 0..face_size {
                    cube.tiles
                        .insert(Position3D(x, y, 1), Position(start_x + x, start_y + y));
                }
            }

            let start_position = Position(*start_x, start_y + face_size);
            if map.area.contains(&start_position) {
                cube.translate(0, -translate, 0);
                cube.rotate(90f64, 0f64, 0f64);
                Self::create_cube(cube, &map, &start_position, visited, face_size);
                cube.rotate(-90f64, 0f64, 0f64);
                cube.translate(0, translate, 0);
            }

            let start_position = Position(*start_x, start_y - face_size);
            if map.area.contains(&start_position) {
                cube.rotate(-90f64, 0f64, 0f64);
                cube.translate(0, translate, 0);
                Self::create_cube(cube, &map, &start_position, visited, face_size);
                cube.translate(0, -translate, 0);
                cube.rotate(90f64, 0f64, 0f64);
            }

            let start_position = Position(start_x + face_size, *start_y);
            if map.area.contains(&start_position) {
                cube.translate(-translate, 0, 0);
                cube.rotate(0f64, -90f64, 0f64);
                Self::create_cube(cube, &map, &start_position, visited, face_size);
                cube.rotate(0f64, 90f64, 0f64);
                cube.translate(translate, 0, 0);
            }

            let start_position = Position(start_x - face_size, *start_y);
            if map.area.contains(&start_position) {
                cube.rotate(0f64, 90f64, 0f64);
                cube.translate(translate, 0, 0);
                Self::create_cube(cube, &map, &start_position, visited, face_size);
                cube.translate(-translate, 0, 0);
                cube.rotate(0f64, -90f64, 0f64);
            }
        }
    }

    fn translate(&mut self, x: i64, y: i64, z: i64) {
        self.tiles = self
            .tiles
            .drain()
            .map(|(Position3D(current_x, current_y, current_z), point2d)| {
                (
                    Position3D(current_x + x, current_y + y, current_z + z),
                    point2d,
                )
            })
            .collect();
    }

    fn rotate(&mut self, roll: f64, pitch: f64, yaw: f64) {
        let cos_alpha = yaw.to_radians().cos();
        let sin_alpha = yaw.to_radians().sin();
        let cos_beta = pitch.to_radians().cos();
        let sin_beta = pitch.to_radians().sin();
        let cos_gamma = roll.to_radians().cos();
        let sin_gamma = roll.to_radians().sin();

        let r_xx = cos_alpha * cos_beta;
        let r_xy = cos_alpha * sin_beta * sin_gamma - sin_alpha * cos_gamma;
        let r_xz = cos_alpha * sin_beta * cos_gamma + sin_alpha * sin_gamma;

        let r_yx = sin_alpha * cos_beta;
        let r_yy = sin_alpha * sin_beta * sin_gamma + cos_alpha * cos_gamma;
        let r_yz = sin_alpha * sin_beta * cos_gamma - cos_alpha * sin_gamma;

        let r_zx = -sin_beta;
        let r_zy = cos_beta * sin_gamma;
        let r_zz = cos_beta * cos_gamma;

        self.tiles = self
            .tiles
            .drain()
            .map(|(Position3D(x, y, z), point2d)| {
                (
                    Position3D(
                        (r_xx * x as f64 + r_xy * y as f64 + r_xz * z as f64).round() as i64,
                        (r_yx * x as f64 + r_yy * y as f64 + r_yz * z as f64).round() as i64,
                        (r_zx * x as f64 + r_zy * y as f64 + r_zz * z as f64).round() as i64,
                    ),
                    point2d,
                )
            })
            .collect();
    }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Position(i64, i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Position3D(i64, i64, i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct PositionAndOrientation {
    position: Position,
    orientation: Orientation,
}

impl PositionAndOrientation {
    fn rotate(&mut self, rotate: char) {
        self.orientation.rotate(rotate);
    }

    fn go(&mut self, distance: i64, map: &Map) {
        let result = (0..distance).try_fold(self.clone(), |previous, _| {
            if let Some(next) = previous.get_next_position_and_orientation(map) {
                ControlFlow::Continue(next)
            } else {
                ControlFlow::Break(previous)
            }
        });
        *self = match result {
            ControlFlow::Continue(next) => next,
            ControlFlow::Break(next) => next,
        };
    }

    fn get_next_position_and_orientation(&self, map: &Map) -> Option<PositionAndOrientation> {
        let next_position = match self.orientation {
            Orientation::East => {
                let (first, last) = Self::get_first_and_last_columns_in_row(self.position.0, map);
                Position(
                    self.position.0,
                    (self.position.1 + 1 - first).rem_euclid(last + 1 - first) + first,
                )
            }
            Orientation::West => {
                let (first, last) = Self::get_first_and_last_columns_in_row(self.position.0, map);
                Position(
                    self.position.0,
                    (self.position.1 - 1 - first).rem_euclid(last + 1 - first) + first,
                )
            }
            Orientation::South => {
                let (first, last) = Self::get_first_and_last_rows_in_column(self.position.1, map);
                Position(
                    (self.position.0 + 1 - first).rem_euclid(last + 1 - first) + first,
                    self.position.1,
                )
            }
            Orientation::North => {
                let (first, last) = Self::get_first_and_last_rows_in_column(self.position.1, map);
                Position(
                    (self.position.0 - 1 - first).rem_euclid(last + 1 - first) + first,
                    self.position.1,
                )
            }
        };

        if map.walls.contains(&next_position) {
            None
        } else {
            Some(PositionAndOrientation {
                position: next_position,
                orientation: self.orientation,
            })
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

#[derive(Debug, Clone)]
struct CubeAndOrientation {
    cube: Cube,
    orientation: Orientation,
    on_mesh_orientation: Orientation,
}

impl CubeAndOrientation {
    fn rotate(&mut self, rotate: char) {
        self.orientation.rotate(rotate);
        self.on_mesh_orientation.rotate(rotate);
    }

    fn go(&mut self, distance: i64, map: &Map) {
        let result = (0..distance).try_fold(self.clone(), |previous, _| {
            if let Some(next) = previous.get_next_position_and_orientation(map) {
                ControlFlow::Continue(next)
            } else {
                ControlFlow::Break(previous)
            }
        });

        *self = match result {
            ControlFlow::Continue(next) => next,
            ControlFlow::Break(next) => next,
        };
    }

    fn get_next_position_and_orientation(&self, map: &Map) -> Option<Self> {
        let (x, y, z) = match self.orientation {
            Orientation::East => (0, -1, 0),
            Orientation::West => (0, 1, 0),
            Orientation::South => (-1, 0, 0),
            Orientation::North => (1, 0, 0),
        };
        let mut next_cube = self.cube.clone();
        next_cube.translate(x, y, z);
        if next_cube.tiles.get(&Position3D(0, 0, 1)).is_none() {
            next_cube = self.cube.clone();
            match self.orientation {
                Orientation::East => next_cube.rotate(90f64, 0f64, 0f64),
                Orientation::West => next_cube.rotate(-90f64, 0f64, 0f64),
                Orientation::South => next_cube.rotate(0f64, -90f64, 0f64),
                Orientation::North => next_cube.rotate(0f64, 90f64, 0f64),
            };
        }

        let next_position = next_cube.tiles.get(&Position3D(0, 0, 1))?;
        let current_position = self.cube.tiles.get(&Position3D(0, 0, 1))?;
        let on_mesh_orientation = match (
            next_position.0 - current_position.0,
            next_position.1 - current_position.1,
        ) {
            (-1, 0) => Orientation::North,
            (1, 0) => Orientation::South,
            (0, 1) => Orientation::East,
            (0, -1) => Orientation::West,
            _ => self.on_mesh_orientation,
        };
        if map.walls.contains(&next_position) {
            None
        } else {
            Some(Self {
                cube: next_cube,
                orientation: self.orientation,
                on_mesh_orientation,
            })
        }
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

fn solve_part_2(file_path: &str) -> i64 {
    let data = load_file(file_path);
    let (map, path) = parse_data(data);
    let cube = Cube::from_map(&map);
    let start_position = map
        .area
        .iter()
        .filter(|position| position.0 == 0 && !map.walls.contains(position))
        .min_by_key(|position| position.1)
        .unwrap();
    let cube_start_position = cube
        .tiles
        .iter()
        .find_map(|(position3d, position2d)| {
            if position2d == start_position {
                Some(position3d)
            } else {
                None
            }
        })
        .unwrap();
    assert_eq!(cube_start_position, &Position3D(0, 0, 1));

    let mut me = CubeAndOrientation {
        cube,
        orientation: Orientation::East,
        on_mesh_orientation: Orientation::East,
    };

    path.instructions
        .into_iter()
        .for_each(|instruction| match instruction {
            PathInstruction::Move(x) => me.go(x, &map),
            PathInstruction::Rotate(x) => me.rotate(x),
        });

    let position = me.cube.tiles.get(&Position3D(0, 0, 1)).unwrap();
    1000 * (position.0 + 1)
        + 4 * (position.1 + 1)
        + match me.on_mesh_orientation {
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
        assert_eq!(result, 6032);
    }

    #[test]
    fn test_part_2() {
        let result = solve_part_2("./resources/test_data.txt");
        assert_eq!(result, 5031);
    }

    #[test]
    fn rotate_roll_plus_90() {
        let mut cube = Cube {
            tiles: HashMap::from([(Position3D(0, 0, 1), Position(0, 0))]),
        };
        cube.rotate(0f64, 90f64, 0f64);
        let expected = HashMap::from([(Position3D(0, -1, 0), Position(0, 0))]);
        assert_eq!(cube.tiles, expected);
    }

    #[test]
    fn rotate_roll_minus_90() {
        let mut cube = Cube {
            tiles: HashMap::from([(Position3D(0, 0, 1), Position(0, 0))]),
        };
        cube.rotate(-90f64, 0f64, 0f64);
        let expected = HashMap::from([(Position3D(0, 1, 0), Position(0, 0))]);
        assert_eq!(cube.tiles, expected);
    }

    #[test]
    fn rotate_pitch_plus_90() {
        let mut cube = Cube {
            tiles: HashMap::from([(Position3D(0, 0, 1), Position(0, 0))]),
        };
        cube.rotate(0f64, 90f64, 0f64);
        let expected = HashMap::from([(Position3D(1, 0, 0), Position(0, 0))]);
        assert_eq!(cube.tiles, expected);
    }

    #[test]
    fn rotate_pitch_minus_90() {
        let mut cube = Cube {
            tiles: HashMap::from([(Position3D(0, 0, 1), Position(0, 0))]),
        };
        cube.rotate(0f64, -90f64, 0f64);
        let expected = HashMap::from([(Position3D(-1, 0, 0), Position(0, 0))]);
        assert_eq!(cube.tiles, expected);
    }

    #[test]
    fn rotate_yaw_plus_90() {
        let mut cube = Cube {
            tiles: HashMap::from([(Position3D(0, 0, 1), Position(0, 0))]),
        };
        cube.rotate(0f64, 0f64, 90f64);
        let expected = HashMap::from([(Position3D(0, 0, 1), Position(0, 0))]);
        assert_eq!(cube.tiles, expected);
    }

    #[test]
    fn rotate_yaw_minus_90() {
        let mut cube = Cube {
            tiles: HashMap::from([(Position3D(0, 0, 1), Position(0, 0))]),
        };
        cube.rotate(0f64, 0f64, -90f64);
        let expected = HashMap::from([(Position3D(0, 0, 1), Position(0, 0))]);
        assert_eq!(cube.tiles, expected);
    }
}
