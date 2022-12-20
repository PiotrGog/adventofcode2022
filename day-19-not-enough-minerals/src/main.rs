use std::{
    fs,
    ops::{Add, Sub},
};

fn load_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

fn parse_data(data: String) -> Vec<Blueprint> {
    data.trim()
        .lines()
        .map(|line| {
            let data = line
                .replace("Blueprint ", "")
                .replace(": Each ore robot costs ", " ")
                .replace(" ore. Each clay robot costs ", " ")
                .replace(" ore. Each obsidian robot costs ", " ")
                .replace(" ore and ", " ")
                .replace(" ore and ", " ")
                .replace(" clay. Each geode robot costs ", " ")
                .replace(" obsidian.", "");
            let mut splitted_data = data.split_ascii_whitespace();
            Blueprint {
                id: splitted_data.next().unwrap().parse().unwrap(),
                ore_robot_cost: Materials {
                    ores: splitted_data.next().unwrap().parse().unwrap(),
                    clays: 0,
                    obsidians: 0,
                    geodes: 0,
                },
                clay_robot_cost: Materials {
                    ores: splitted_data.next().unwrap().parse().unwrap(),
                    clays: 0,
                    obsidians: 0,
                    geodes: 0,
                },
                obsidian_robot_cost: Materials {
                    ores: splitted_data.next().unwrap().parse().unwrap(),
                    clays: splitted_data.next().unwrap().parse().unwrap(),
                    obsidians: 0,
                    geodes: 0,
                },
                geode_robot_cost: Materials {
                    ores: splitted_data.next().unwrap().parse().unwrap(),
                    clays: 0,
                    obsidians: splitted_data.next().unwrap().parse().unwrap(),
                    geodes: 0,
                },
            }
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Blueprint {
    id: usize,
    ore_robot_cost: Materials,
    clay_robot_cost: Materials,
    obsidian_robot_cost: Materials,
    geode_robot_cost: Materials,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Materials {
    ores: usize,
    clays: usize,
    obsidians: usize,
    geodes: usize,
}

impl Add for Materials {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            ores: self.ores + rhs.ores,
            clays: self.clays + rhs.clays,
            obsidians: self.obsidians + rhs.obsidians,
            geodes: self.geodes + rhs.geodes,
        }
    }
}

impl Sub for Materials {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            ores: self.ores - rhs.ores,
            clays: self.clays - rhs.clays,
            obsidians: self.obsidians - rhs.obsidians,
            geodes: self.geodes - rhs.geodes,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Robots {
    ore_robots: usize,
    clay_robots: usize,
    obsidian_robots: usize,
    geode_robots: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Status {
    blueprint: Blueprint,
    robots: Robots,
    materials: Materials,
}

impl Add for Robots {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            ore_robots: self.ore_robots + rhs.ore_robots,
            clay_robots: self.clay_robots + rhs.clay_robots,
            obsidian_robots: self.obsidian_robots + rhs.obsidian_robots,
            geode_robots: self.geode_robots + rhs.geode_robots,
        }
    }
}

impl Sub for Robots {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            ore_robots: self.ore_robots - rhs.ore_robots,
            clay_robots: self.clay_robots - rhs.clay_robots,
            obsidian_robots: self.obsidian_robots - rhs.obsidian_robots,
            geode_robots: self.geode_robots - rhs.geode_robots,
        }
    }
}

impl Robots {
    fn collect_materials(&self) -> Materials {
        Materials {
            ores: self.ore_robots,
            clays: self.clay_robots,
            obsidians: self.obsidian_robots,
            geodes: self.geode_robots,
        }
    }
}

impl Status {
    fn new(blueprint: Blueprint) -> Self {
        Self {
            blueprint,
            robots: Robots {
                ore_robots: 1,
                clay_robots: 0,
                obsidian_robots: 0,
                geode_robots: 0,
            },
            materials: Materials {
                ores: 0,
                clays: 0,
                obsidians: 0,
                geodes: 0,
            },
        }
    }
}

struct Solver;

impl Solver {
    pub fn solve(minutes_left: u8, status: Status) -> usize {
        let Status {
            blueprint,
            mut robots,
            materials,
        } = status;
        if minutes_left == 0 {
            return materials.geodes;
        }
        let collected_materials = robots.collect_materials();
        let mut max_geode = 0;

        let need_more_obsidian_robots =
            robots.obsidian_robots < blueprint.geode_robot_cost.obsidians;
        let need_more_clay_robots = robots.clay_robots < blueprint.obsidian_robot_cost.clays;
        let need_more_ore_robots = robots.ore_robots
            < [
                blueprint.ore_robot_cost.ores,
                blueprint.clay_robot_cost.ores,
                blueprint.obsidian_robot_cost.ores,
                blueprint.geode_robot_cost.ores,
            ]
            .into_iter()
            .max()
            .unwrap();

        if Self::can_build_geode_robot(&blueprint, &materials) {
            let mut robots = robots.clone();
            robots.geode_robots += 1;
            max_geode = max_geode.max(Self::solve(
                minutes_left - 1,
                Status {
                    blueprint,
                    robots,
                    materials: materials + collected_materials - blueprint.geode_robot_cost,
                },
            ));
            return max_geode;
        } else if Self::can_build_obsidian_robot(&blueprint, &materials)
            && need_more_obsidian_robots
        {
            let mut robots = robots.clone();
            robots.obsidian_robots += 1;
            max_geode = max_geode.max(Self::solve(
                minutes_left - 1,
                Status {
                    blueprint,
                    robots,
                    materials: materials + collected_materials - blueprint.obsidian_robot_cost,
                },
            ));
        } else {
            if Self::can_build_clay_robot(&blueprint, &materials) && need_more_clay_robots {
                let mut robots = robots.clone();
                robots.clay_robots += 1;
                max_geode = max_geode.max(Self::solve(
                    minutes_left - 1,
                    Status {
                        blueprint,
                        robots,
                        materials: materials + collected_materials - blueprint.clay_robot_cost,
                    },
                ));
            }
            if Self::can_build_ore_robot(&blueprint, &materials) && need_more_ore_robots {
                let mut robots = robots.clone();
                robots.ore_robots += 1;
                max_geode = max_geode.max(Self::solve(
                    minutes_left - 1,
                    Status {
                        blueprint,
                        robots,
                        materials: materials + collected_materials - blueprint.ore_robot_cost,
                    },
                ));
            }
            {
                max_geode = max_geode.max(Self::solve(
                    minutes_left - 1,
                    Status {
                        blueprint,
                        robots,
                        materials: materials + collected_materials,
                    },
                ));
            }
        };

        max_geode
    }

    fn can_build_geode_robot(blueprint: &Blueprint, materials: &Materials) -> bool {
        Self::enough_resources(&blueprint.geode_robot_cost, materials)
    }

    fn can_build_obsidian_robot(blueprint: &Blueprint, materials: &Materials) -> bool {
        Self::enough_resources(&blueprint.obsidian_robot_cost, materials)
    }

    fn can_build_clay_robot(blueprint: &Blueprint, materials: &Materials) -> bool {
        Self::enough_resources(&blueprint.clay_robot_cost, materials)
    }

    fn can_build_ore_robot(blueprint: &Blueprint, materials: &Materials) -> bool {
        Self::enough_resources(&blueprint.ore_robot_cost, materials)
    }

    fn enough_resources(needed_materials: &Materials, materials: &Materials) -> bool {
        materials.ores >= needed_materials.ores
            && materials.clays >= needed_materials.clays
            && materials.obsidians >= needed_materials.obsidians
    }
}

fn solve_part_1(file_path: &str) -> usize {
    let data = load_file(file_path);
    let blueprints = parse_data(data);
    let statuses = blueprints
        .into_iter()
        .map(|blueprint| Status::new(blueprint));
    statuses
        .into_iter()
        .map(|status| status.blueprint.id * Solver::solve(24, status))
        .sum()
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
        assert_eq!(result, 33);
    }

    #[test]
    fn test_solver() {
        let data = load_file("./resources/test_data.txt");
        let blueprints = parse_data(data);
        let result = Solver::solve(24, Status::new(blueprints[0]));
        assert_eq!(result, 9);
    }

    #[test]
    fn test_load_data() {
        let data = load_file("./resources/test_data.txt");
        let blueprints = parse_data(data);
        assert_eq!(
            blueprints,
            vec![
                Blueprint {
                    id: 1,
                    ore_robot_cost: Materials {
                        ores: 4,
                        clays: 0,
                        obsidians: 0,
                        geodes: 0,
                    },
                    clay_robot_cost: Materials {
                        ores: 2,
                        clays: 0,
                        obsidians: 0,
                        geodes: 0,
                    },
                    obsidian_robot_cost: Materials {
                        ores: 3,
                        clays: 14,
                        obsidians: 0,
                        geodes: 0,
                    },
                    geode_robot_cost: Materials {
                        ores: 2,
                        clays: 0,
                        obsidians: 7,
                        geodes: 0,
                    },
                },
                Blueprint {
                    id: 2,
                    ore_robot_cost: Materials {
                        ores: 2,
                        clays: 0,
                        obsidians: 0,
                        geodes: 0,
                    },
                    clay_robot_cost: Materials {
                        ores: 3,
                        clays: 0,
                        obsidians: 0,
                        geodes: 0,
                    },
                    obsidian_robot_cost: Materials {
                        ores: 3,
                        clays: 8,
                        obsidians: 0,
                        geodes: 0,
                    },
                    geode_robot_cost: Materials {
                        ores: 3,
                        clays: 0,
                        obsidians: 12,
                        geodes: 0,
                    },
                }
            ]
        )
    }
}
