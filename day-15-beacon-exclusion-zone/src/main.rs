use std::{collections::HashSet, fs};

fn load_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

fn parse_data(data: String) -> Vec<Sensor> {
    data.trim()
        .lines()
        .map(|line| {
            let cleaned_line = line
                .replace("Sensor at x=", "")
                .replace(", y=", " ")
                .replace(": closest beacon is at x=", " ");
            let coordinates_vec = cleaned_line.split_whitespace().collect::<Vec<_>>();
            let sensor_position = Position {
                x: coordinates_vec[0].parse().unwrap(),
                y: coordinates_vec[1].parse().unwrap(),
            };
            let beacon_position = Position {
                x: coordinates_vec[2].parse().unwrap(),
                y: coordinates_vec[3].parse().unwrap(),
            };
            let scanned_distance = sensor_position.manhatan_distance(&beacon_position);
            Sensor {
                position: sensor_position,
                found_beacon: beacon_position,
                scanned_distance,
            }
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn manhatan_distance(&self, other: &Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Sensor {
    position: Position,
    found_beacon: Position,
    scanned_distance: usize,
}

fn get_beacons_positions(sensors: &Vec<Sensor>) -> HashSet<Position> {
    sensors.iter().map(|sensor| sensor.found_beacon).collect()
}

fn count_points_without_beacon_in_row(sensors: Vec<Sensor>, row_to_check: isize) -> usize {
    let sensors_which_scanned_row = sensors
        .iter()
        .filter_map(|sensor| {
            if sensor.position.y.abs_diff(row_to_check) <= sensor.scanned_distance {
                Some(sensor)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let scanned_intervals =
        sensors_which_scanned_row
            .iter()
            .fold(Vec::new(), |mut intervals, sensor| {
                let distance_along_row =
                    (sensor.scanned_distance - sensor.position.y.abs_diff(row_to_check)) as isize;
                intervals.push(Interval {
                    begin: sensor.position.x - distance_along_row,
                    end: sensor.position.x + distance_along_row,
                });
                intervals
            });
    let merged_intervals = Interval::merge(scanned_intervals);
    let counted_beacons_in_a_row = get_beacons_positions(&sensors)
        .into_iter()
        .filter_map(|beacon| {
            if beacon.y == row_to_check
                && merged_intervals
                    .iter()
                    .any(|interval| interval.point_in(beacon.x))
            {
                Some(beacon.x)
            } else {
                None
            }
        })
        .count();
    let intervals_full_length: usize = merged_intervals.iter().map(|interval| interval.len()).sum();

    intervals_full_length - counted_beacons_in_a_row
}

fn find_beacon_frequency(sensors: Vec<Sensor>, max_rows: usize) -> usize {
    let mut invervals_in_row = vec![vec![]; max_rows + 1];
    for row_to_check in 0..=max_rows {
        let sensors_which_scanned_row = sensors
            .iter()
            .filter_map(|sensor| {
                if sensor.position.y.abs_diff(row_to_check as isize) <= sensor.scanned_distance {
                    Some(sensor)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        let scanned_intervals =
            sensors_which_scanned_row
                .iter()
                .fold(Vec::new(), |mut intervals, sensor| {
                    let distance_along_row = (sensor.scanned_distance
                        - sensor.position.y.abs_diff(row_to_check as isize))
                        as isize;
                    intervals.push(Interval {
                        begin: sensor.position.x - distance_along_row,
                        end: sensor.position.x + distance_along_row,
                    });
                    intervals
                });
        let merged_intervals = Interval::merge(scanned_intervals);
        invervals_in_row[row_to_check].extend(merged_intervals);
    }

    let result = invervals_in_row
        .into_iter()
        .map(Interval::merge)
        .enumerate()
        .filter(|(_, row_intervals)| {
            row_intervals.iter().any(|interval| {
                (interval.begin > 0 && interval.begin < max_rows as isize)
                    | (interval.end > 0 && interval.end < max_rows as isize)
            })
        })
        .collect::<Vec<_>>();

    let (row, intervals) = &result[0];
    (intervals[0].end as usize + 1) * 4000000 + row
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Interval {
    begin: isize,
    end: isize,
}

impl Interval {
    fn len(&self) -> usize {
        self.end.abs_diff(self.begin) + 1
    }

    fn point_in(&self, point: isize) -> bool {
        point >= self.begin && point <= self.end
    }

    fn merge(mut intervals: Vec<Interval>) -> Vec<Interval> {
        intervals.sort_by_key(|interval| interval.begin);

        let init = vec![intervals[0].clone()];
        intervals
            .into_iter()
            .fold(init, |mut merged_intervals, interval| {
                if let Some(last_interval) = merged_intervals.last_mut() {
                    if interval.begin <= last_interval.end + 1 {
                        last_interval.end = last_interval.end.max(interval.end);
                    } else {
                        merged_intervals.push(interval);
                    }
                }

                merged_intervals
            })
    }
}

fn solve_part_1(file_path: &str) -> usize {
    let data = load_file(file_path);
    let sensors = parse_data(data);
    count_points_without_beacon_in_row(sensors, 2000000)
}

fn solve_part_2(file_path: &str) -> usize {
    let data = load_file(file_path);
    let sensors = parse_data(data);
    find_beacon_frequency(sensors, 4000000)
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
        let data = load_file("./resources/test_data.txt");
        let sensors = parse_data(data);
        let result = count_points_without_beacon_in_row(sensors, 10);
        assert_eq!(result, 26);
    }

    #[test]
    fn test_part_2() {
        let data = load_file("./resources/test_data.txt");
        let sensors = parse_data(data);
        let result = find_beacon_frequency(sensors, 20);
        assert_eq!(result, 56000011);
    }

    #[test]
    fn test_parse_data() {
        let data = load_file("./resources/test_data.txt");
        let data = parse_data(data);
        let expected = vec![
            Sensor {
                position: Position { x: 2, y: 18 },
                found_beacon: Position { x: -2, y: 15 },
                scanned_distance: 7,
            },
            Sensor {
                position: Position { x: 9, y: 16 },
                found_beacon: Position { x: 10, y: 16 },
                scanned_distance: 1,
            },
            Sensor {
                position: Position { x: 13, y: 2 },
                found_beacon: Position { x: 15, y: 3 },
                scanned_distance: 3,
            },
            Sensor {
                position: Position { x: 12, y: 14 },
                found_beacon: Position { x: 10, y: 16 },
                scanned_distance: 4,
            },
            Sensor {
                position: Position { x: 10, y: 20 },
                found_beacon: Position { x: 10, y: 16 },
                scanned_distance: 4,
            },
            Sensor {
                position: Position { x: 14, y: 17 },
                found_beacon: Position { x: 10, y: 16 },
                scanned_distance: 5,
            },
            Sensor {
                position: Position { x: 8, y: 7 },
                found_beacon: Position { x: 2, y: 10 },
                scanned_distance: 9,
            },
            Sensor {
                position: Position { x: 2, y: 0 },
                found_beacon: Position { x: 2, y: 10 },
                scanned_distance: 10,
            },
            Sensor {
                position: Position { x: 0, y: 11 },
                found_beacon: Position { x: 2, y: 10 },
                scanned_distance: 3,
            },
            Sensor {
                position: Position { x: 20, y: 14 },
                found_beacon: Position { x: 25, y: 17 },
                scanned_distance: 8,
            },
            Sensor {
                position: Position { x: 17, y: 20 },
                found_beacon: Position { x: 21, y: 22 },
                scanned_distance: 6,
            },
            Sensor {
                position: Position { x: 16, y: 7 },
                found_beacon: Position { x: 15, y: 3 },
                scanned_distance: 5,
            },
            Sensor {
                position: Position { x: 14, y: 3 },
                found_beacon: Position { x: 15, y: 3 },
                scanned_distance: 1,
            },
            Sensor {
                position: Position { x: 20, y: 1 },
                found_beacon: Position { x: 15, y: 3 },
                scanned_distance: 7,
            },
        ];
        assert_eq!(data, expected);
    }
}
