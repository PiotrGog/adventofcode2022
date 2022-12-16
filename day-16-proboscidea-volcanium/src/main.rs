use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

fn load_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

fn parse_data(data: String) -> HashMap<String, (u32, Vec<String>)> {
    data.trim()
        .lines()
        .map(|line| {
            let cleaned_line = line
                .replace("Valve ", "")
                .replace(" has flow rate=", " ")
                .replace("; tunnels lead to valves ", " ")
                .replace("; tunnel leads to valve ", " ")
                .replace(", ", ",");
            let data = cleaned_line.split_whitespace().collect::<Vec<_>>();
            let valve = data[0].to_string();
            let flow_rate: u32 = data[1].parse().unwrap();

            let lead_to = data[2].split(",").map(String::from).collect::<Vec<_>>();
            (valve, (flow_rate, lead_to))
        })
        .collect()
}

fn find_shortest_path(
    data: &HashMap<String, (u32, Vec<String>)>,
    start: &String,
    stop: &String,
) -> Option<u32> {
    let mut queue = VecDeque::from([(start, 0)]);
    let mut visited = HashSet::from([start]);

    while let Some((node, step)) = queue.pop_front() {
        if node == stop {
            return Some(step);
        }
        data.get(node).unwrap().1.iter().for_each(|next_node| {
            if visited.insert(next_node) {
                queue.push_back((next_node, step + 1));
            }
        });
    }

    None
}

fn solve(
    data: &HashMap<String, (u32, Vec<String>)>,
    current_valve: &String,
    available_minutes: i32,
    valves_to_open: HashMap<&String, &(u32, Vec<String>)>,
) -> i32 {
    let mut queue = VecDeque::from([(current_valve, valves_to_open, available_minutes, 0)]);

    let mut max_flow = 0;

    let mut con = HashMap::new();

    while let Some((valve, valves_to_open, available_minutes, current_flow)) = queue.pop_front() {
        let mut vto: Vec<_> = valves_to_open.iter().map(|(n, _)| n.clone()).collect();
        vto.sort();
        if let Some(r) = con.get(&vto) {
            if r > &current_flow {
                continue;
            }
        }

        con.insert(vto, current_flow);

        for (next_valve, (flow, _)) in &valves_to_open {
            if let Some(distance) = find_shortest_path(data, valve, next_valve) {
                let available_minutes = available_minutes - distance as i32 - 1;
                if available_minutes > 2 {
                    let mut valves_to_open = valves_to_open.clone();
                    valves_to_open.remove(next_valve);
                    if valves_to_open.is_empty() {
                        max_flow = max_flow.max(current_flow + (available_minutes * *flow as i32));
                        continue;
                    }
                    queue.push_back((
                        next_valve,
                        valves_to_open,
                        available_minutes,
                        current_flow + (available_minutes * *flow as i32),
                    ))
                } else {
                    max_flow = max_flow.max(current_flow);
                }
            }
        }
    }

    max_flow
}

fn solve_part_1(file_path: &str) -> u32 {
    let data = load_file(file_path);
    let data = parse_data(data);
    let available_minutes = 30;
    let start_valve = "AA".to_string();
    let valves_to_open: HashMap<_, _> = data.iter().filter(|(_, (flow, _))| flow > &0).collect();
    solve(
        &data,
        &start_valve.to_string(),
        available_minutes,
        valves_to_open,
    ) as u32
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
        assert_eq!(result, 1651);
    }
}
