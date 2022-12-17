use std::{collections::HashSet, fs};

fn load_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

fn generate_jets<'a>(data: &'a String) -> impl Iterator<Item = char> + 'a {
    data.chars().cycle()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Tower {
    height: usize,
    width: usize,
    rocks: HashSet<(usize, usize)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Rock {
    rock: Vec<(usize, usize)>,
}

impl Rock {
    pub fn new_horizontal(hight: usize) -> Self {
        Self {
            rock: vec![(2, hight), (3, hight), (4, hight), (5, hight)],
        }
    }

    pub fn new_vertical(hight: usize) -> Self {
        Self {
            rock: vec![
                (2, 0 + hight),
                (2, 1 + hight),
                (2, 2 + hight),
                (2, 3 + hight),
            ],
        }
    }

    pub fn new_cross(hight: usize) -> Self {
        Self {
            rock: vec![
                (3, 0 + hight),
                (2, 1 + hight),
                (3, 1 + hight),
                (4, 1 + hight),
                (3, 2 + hight),
            ],
        }
    }

    pub fn new_cube(hight: usize) -> Self {
        Self {
            rock: vec![
                (2, 0 + hight),
                (3, 0 + hight),
                (2, 1 + hight),
                (3, 1 + hight),
            ],
        }
    }

    pub fn new_corner(hight: usize) -> Self {
        Self {
            rock: vec![
                (2, 0 + hight),
                (3, 0 + hight),
                (4, 0 + hight),
                (4, 1 + hight),
                (4, 2 + hight),
            ],
        }
    }

    pub fn move_down(&self) -> Self {
        Self {
            rock: self
                .rock
                .iter()
                .map(|(coord_x, coord_y)| (*coord_x, coord_y - 1))
                .collect(),
        }
    }
    pub fn move_right(&self) -> Self {
        Self {
            rock: self
                .rock
                .iter()
                .map(|(coord_x, coord_y)| (coord_x + 1, *coord_y))
                .collect(),
        }
    }
    pub fn move_left(&self) -> Self {
        Self {
            rock: self
                .rock
                .iter()
                .map(|(coord_x, coord_y)| (coord_x - 1, *coord_y))
                .collect(),
        }
    }

    pub fn move_down_if_possible(&mut self, tower: &Tower) -> bool {
        if self.rock.iter().all(|e| e.1 > 0) {
            let tmp_rock = self.move_down();
            if tmp_rock.rock.iter().all(|e| !tower.rocks.contains(e)) {
                *self = tmp_rock;
                return true;
            }
        }
        false
    }

    pub fn move_right_if_possible(&mut self, tower: &Tower) -> bool {
        if self.rock.iter().all(|e| e.0 < tower.width - 1) {
            let tmp_rock = self.move_right();
            if tmp_rock.rock.iter().all(|e| !tower.rocks.contains(e)) {
                *self = tmp_rock;
                return true;
            }
        }
        false
    }

    pub fn move_left_if_possible(&mut self, tower: &Tower) -> bool {
        if self.rock.iter().all(|e| e.0 > 0) {
            let rock_tmp = self.move_left();
            if rock_tmp.rock.iter().all(|e| !tower.rocks.contains(e)) {
                *self = rock_tmp;
                return true;
            }
        }
        false
    }

    pub fn rocks_generator(num: usize) -> impl FnOnce(usize) -> Rock {
        [
            Self::new_horizontal,
            Self::new_cross,
            Self::new_corner,
            Self::new_vertical,
            Self::new_cube,
        ][num % 5]
    }
}

fn drop_rock(mut rock: Rock, jet_generator: &mut impl Iterator<Item = char>, tower: &mut Tower) {
    let mut running = true;
    while running {
        match jet_generator.next() {
            Some('>') => rock.move_right_if_possible(&tower),
            Some('<') => rock.move_left_if_possible(&tower),
            other => panic!("unsupported {other:?}"),
        };

        if !rock.move_down_if_possible(&tower) {
            tower.rocks.extend(rock.rock.iter());
            tower.height = tower.rocks.iter().map(|e| e.1).max().unwrap() + 1;
            running = false;
        }
    }
}

fn solve_part_1(file_path: &str) -> usize {
    let mut tower = Tower {
        height: 0,
        width: 7,
        rocks: HashSet::new(),
    };

    let rocks_to_throw = 2022;

    let data = load_file(file_path).trim().to_string();
    let mut jet_generator = generate_jets(&data);
    for i in 0..rocks_to_throw {
        let rock = Rock::rocks_generator(i)(tower.height + 3);
        drop_rock(rock, &mut jet_generator, &mut tower);
    }

    tower.height
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
        assert_eq!(result, 3068);
    }
}
