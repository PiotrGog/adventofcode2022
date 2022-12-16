use std::{
    collections::{HashMap, HashSet},
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

fn solve(
    data: &HashMap<String, (u32, Vec<String>)>,
    current_valve: &String,
    available_minutes: i32,
    current_flow: u32,
    turned_on: HashSet<String>,
    conf: &mut HashMap<(String, i32, bool), u32>,
) -> u32 {
    if available_minutes <= 0 {
        return current_flow;
    }

    let current_valve_is_not_turned_on = !turned_on.contains(current_valve);
    if let Some(v) = conf.get(&(
        current_valve.to_string(),
        available_minutes,
        current_valve_is_not_turned_on,
    )) {
        if v >= &current_flow {
            return *v;
        }
    }

    let mut max_flow = u32::MIN;

    let (flow_rate, next_valves) = data.get(current_valve).unwrap();
    let available_minutes_after_move = available_minutes - 1;
    if flow_rate > &0 && current_valve_is_not_turned_on {
        let mut new_tuned_on = turned_on.clone();
        new_tuned_on.insert(current_valve.clone());

        max_flow = max_flow.max(solve(
            data,
            current_valve,
            available_minutes_after_move,
            current_flow + (available_minutes_after_move as u32 * flow_rate),
            new_tuned_on,
            conf,
        ));
    }
    for next_valve in next_valves {
        max_flow = max_flow.max(solve(
            data,
            next_valve,
            available_minutes_after_move,
            current_flow,
            turned_on.clone(),
            conf,
        ));
    }

    conf.insert(
        (
            current_valve.to_string(),
            available_minutes,
            current_valve_is_not_turned_on,
        ),
        current_flow,
    );

    max_flow
}

fn solve_part_1(file_path: &str) -> u32 {
    let data = load_file(file_path);
    let valves_with_flow_rate_and_leads_to = parse_data(data);
    let available_minutes = 30;
    let start_valve = "AA";
    solve(
        &valves_with_flow_rate_and_leads_to,
        &start_valve.to_string(),
        available_minutes,
        0,
        HashSet::new(),
        &mut HashMap::new(),
    )
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
