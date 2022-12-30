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

fn solve_2(
    data: &HashMap<String, (u32, Vec<String>)>,
    current_valve: &String,
    available_minutes: i32,
    valves_to_open: HashMap<&String, &(u32, Vec<String>)>,
    con: &mut HashMap<Vec<String>, (i32, i32)>,
) -> i32 {
    let mut queue = VecDeque::from([(current_valve, valves_to_open, available_minutes, 0)]);

    let mut max_flow = 0;

    while let Some((valve, valves_to_open, available_minutes, current_flow)) = queue.pop_front() {
        let mut vto: Vec<String> = valves_to_open
            .iter()
            .map(|(n, _)| n.to_owned().clone())
            .collect();
        vto.sort();
        if let Some(r) = con.get(&vto) {
            if r.0 > current_flow && r.1 > available_minutes {
                continue;
            }
        }

        con.insert(vto, (current_flow, available_minutes));

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

fn partition_sub<Type>(
    arr: &Vec<Type>,
    i: usize,
    n: usize,
    k: usize,
    nos: usize,
    v: &mut Vec<VecDeque<Type>>,
    result: &mut Vec<Vec<VecDeque<Type>>>,
) where
    Type: Clone,
{
    if i >= arr.len() {
        result.push(v.clone());
        return;
    }

    for j in 0..k {
        if v[j].len() > 0 {
            v[j].push_back(arr[i].clone());
            partition_sub(arr, i + 1, n, k, nos, v, result);
            v[j].pop_back();
        } else {
            v[j].push_back(arr[i].clone());
            partition_sub(arr, i + 1, n, k, nos + 1, v, result);
            v[j].pop_back();
            break;
        }
    }
}

fn part_k_subsets<Type>(arr: &Vec<Type>, k: usize) -> Vec<Vec<VecDeque<Type>>>
where
    Type: Clone,
{
    let n = arr.len();
    let mut v = vec![VecDeque::new(); k];
    let mut result = Vec::new();
    if k != 0 && k <= n {
        partition_sub(arr, 0, n, k, 0, &mut v, &mut result);
    }
    result
}

fn solve_part_2(file_path: &str) -> u32 {
    let data = load_file(file_path);
    let data = parse_data(data);
    let available_minutes = 26;
    let start_valve = "AA".to_string();
    let valves_to_open: HashMap<_, _> = data.iter().filter(|(_, (flow, _))| flow > &0).collect();

    let subsets = part_k_subsets(&valves_to_open.into_iter().collect(), 2);
    let mut max_flow = 0;
    let mut con = HashMap::new();
    for sets in subsets.iter().filter(|s| s[0].len() * 2 < 20) {
        let result1 = solve_2(
            &data,
            &start_valve.to_string(),
            available_minutes,
            sets[0].iter().cloned().collect(),
            &mut con,
        ) as u32;
        let result2 = solve_2(
            &data,
            &start_valve.to_string(),
            available_minutes,
            sets[1].iter().cloned().collect(),
            &mut con,
        ) as u32;
        max_flow = max_flow.max(result1 + result2);
    }

    max_flow
}

fn part_1(file_path: &str) {
    let result = solve_part_1(file_path);
    println!("Part 1 result: {:?}", result);
}

fn part_2(file_path: &str) {
    let result = solve_part_2(file_path);
    println!("Part 2 result: {:?}    2469", result);
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
        assert_eq!(result, 1651);
    }

    #[test]
    fn test_part_2() {
        let result = solve_part_2("./resources/test_data.txt");
        assert_eq!(result, 1707);
    }
}
