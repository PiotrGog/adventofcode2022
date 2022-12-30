use std::{collections::HashSet, fs};

fn load_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

fn parse_data(data: String) -> HashSet<Cube> {
    data.trim()
        .lines()
        .map(|line| {
            let data = line.split(',').collect::<Vec<_>>();
            let x = data[0].parse().unwrap();
            let y = data[1].parse().unwrap();
            let z = data[2].parse().unwrap();
            Cube { x, y, z }
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Cube {
    x: isize,
    y: isize,
    z: isize,
}

fn get_cube_neighborhood(cube: &Cube) -> HashSet<Cube> {
    let Cube { x, y, z } = cube;
    HashSet::from_iter(
        [
            Some(Cube { x: x - 1, ..*cube }),
            Some(Cube { y: y - 1, ..*cube }),
            Some(Cube { z: z - 1, ..*cube }),
            Some(Cube { x: x + 1, ..*cube }),
            Some(Cube { y: y + 1, ..*cube }),
            Some(Cube { z: z + 1, ..*cube }),
        ]
        .into_iter()
        .flatten(),
    )
}

fn solve_part_1(file_path: &str) -> usize {
    let data = load_file(file_path);
    let cubes = parse_data(data);
    cubes.iter().fold(0, |exposed_surfaces_sum, cube| {
        exposed_surfaces_sum
            + get_cube_neighborhood(cube)
                .into_iter()
                .filter(|neighborhood_cube| !cubes.contains(neighborhood_cube))
                .count()
    })
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
        assert_eq!(result, 64);
    }
}
