use std::{
    collections::{HashSet, VecDeque},
    fs,
};

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

fn find_super_cube_corners(cubes: &HashSet<Cube>) -> (Cube, Cube) {
    let init = (
        Cube {
            x: isize::MAX,
            y: isize::MAX,
            z: isize::MAX,
        },
        Cube {
            x: isize::MIN,
            y: isize::MIN,
            z: isize::MIN,
        },
    );
    cubes.iter().fold(init, |(cube_min, cube_max), cube| {
        let Cube { x, y, z } = cube;
        let Cube {
            x: x_min,
            y: y_min,
            z: z_min,
        } = cube_min;
        let Cube {
            x: x_max,
            y: y_max,
            z: z_max,
        } = cube_max;
        (
            Cube {
                x: x_min.min(*x),
                y: y_min.min(*y),
                z: z_min.min(*z),
            },
            Cube {
                x: x_max.max(*x),
                y: y_max.max(*y),
                z: z_max.max(*z),
            },
        )
    })
}

fn solve_part_2(file_path: &str) -> usize {
    let data = load_file(file_path);
    let cubes = parse_data(data);
    let (cube_min, cube_max) = find_super_cube_corners(&cubes);
    let (cube_min, cube_max) = (
        Cube {
            x: cube_min.x - 1,
            y: cube_min.y - 1,
            z: cube_min.z - 1,
        },
        Cube {
            x: cube_max.x + 1,
            y: cube_max.y + 1,
            z: cube_max.z + 1,
        },
    );
    let source = cube_min.clone();
    let mut queue = VecDeque::from([source]);
    let mut visited = HashSet::from([source]);

    while let Some(cube) = queue.pop_front() {
        let cubes_in = get_cube_neighborhood(&cube)
            .into_iter()
            .filter(|neighborhood_cube| {
                !cubes.contains(neighborhood_cube)
                    && !visited.contains(neighborhood_cube)
                    && neighborhood_cube.x >= cube_min.x
                    && neighborhood_cube.x <= cube_max.x
                    && neighborhood_cube.y >= cube_min.y
                    && neighborhood_cube.y <= cube_max.y
                    && neighborhood_cube.z >= cube_min.z
                    && neighborhood_cube.z <= cube_max.z
            })
            .collect::<Vec<_>>();
        for cube_in in cubes_in {
            visited.insert(cube_in);
            queue.push_back(cube_in);
        }
    }
    cubes.iter().fold(0, |exposed_surfaces_sum, cube| {
        exposed_surfaces_sum
            + get_cube_neighborhood(cube)
                .into_iter()
                .filter(|neighborhood_cube| visited.contains(neighborhood_cube))
                .count()
    })
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
        assert_eq!(result, 64);
    }

    #[test]
    fn test_part_2() {
        let result = solve_part_2("./resources/test_data.txt");
        assert_eq!(result, 58);
    }
}
