use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn load_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

fn jets(data: &String) -> Vec<char> {
    data.chars().collect()
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

fn drop_rock(mut rock: Rock, jets: &Vec<char>, tower: &mut Tower, time: &mut usize) {
    let mut running = true;
    while running {
        match jets[*time % jets.len()] {
            '>' => rock.move_right_if_possible(&tower),
            '<' => rock.move_left_if_possible(&tower),
            other => panic!("unsupported {other:?}"),
        };

        if !rock.move_down_if_possible(&tower) {
            tower.rocks.extend(rock.rock.iter());
            tower.height = tower.rocks.iter().map(|e| e.1).max().unwrap() + 1;
            running = false;
        }
        *time += 1;
    }
}

fn signature(tower: &Tower) -> u64 {
    let mut mask = 1;
    let mut sig = 0;
    for i in 0..10 {
        for j in 0..tower.width {
            let coords = (j, tower.height - i);
            if tower.rocks.contains(&coords) {
                sig |= mask;
            }
            mask <<= 1;
        }
        if tower.height - i == 0 {
            break;
        }
    }
    sig
}

fn solve_part_1(file_path: &str) -> usize {
    let mut tower = Tower {
        height: 0,
        width: 7,
        rocks: HashSet::new(),
    };

    let rocks_to_throw = 2022;

    let data = load_file(file_path).trim().to_string();
    let jets = jets(&data);
    let mut time = 0;
    for i in 0..rocks_to_throw {
        let rock = Rock::rocks_generator(i)(tower.height + 3);
        drop_rock(rock, &jets, &mut tower, &mut time);
    }

    tower.height
}

fn solve_part_2(file_path: &str) -> usize {
    let mut tower = Tower {
        height: 0,
        width: 7,
        rocks: HashSet::new(),
    };

    let rocks_to_throw = 1000000000000;

    let data = load_file(file_path).trim().to_string();
    let jets = jets(&data);
    let mut time = 0;
    let mut skipped_add = 0;
    let mut i = 0;

    let mut precomputed = HashMap::<(u64, usize, usize), (usize, usize)>::new();

    while i < rocks_to_throw {
        let rock = Rock::rocks_generator(i)(tower.height + 3);
        drop_rock(rock, &jets, &mut tower, &mut time);
        time %= jets.len();
        let sign = signature(&tower);

        let signature_key = (sign, time, i % 5);
        if let Some((previous_height, rock_num)) = precomputed.get(&signature_key) {
            let rocks_diff = i - rock_num;
            let height_diff = tower.height - previous_height;
            let to_max = rocks_to_throw - i;
            i += (to_max / rocks_diff) * rocks_diff;
            skipped_add += (to_max / rocks_diff) * height_diff;
        } else {
            precomputed.insert(signature_key, (tower.height, i));
        }
        i += 1;
    }

    tower.height + skipped_add
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
        assert_eq!(result, 3068);
    }

    #[test]
    fn test_part_2() {
        let result = solve_part_2("./resources/test_data.txt");
        assert_eq!(result, 1514285714288);
    }
}
