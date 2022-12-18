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
        let mut new_turned_on = turned_on.clone();
        new_turned_on.insert(current_valve.clone());

        max_flow = max_flow.max(solve(
            data,
            current_valve,
            available_minutes_after_move,
            current_flow + (available_minutes_after_move as u32 * flow_rate),
            new_turned_on,
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

fn solve_with_elephant(
    data: &HashMap<String, (u32, Vec<String>)>,
    my_current_valve: &String,
    elephant_current_valve: &String,
    available_minutes: i32,
    current_flow: u32,
    turned_on: HashSet<String>,
    conf: &mut HashMap<(String, String, i32, bool, bool), u32>,
) -> u32 {
    if available_minutes <= 0 {
        return current_flow;
    }

    let my_current_valve_is_not_turned_on = !turned_on.contains(my_current_valve);
    let elephant_current_valve_is_not_turned_on = !turned_on.contains(elephant_current_valve);
    if let Some(v) = conf.get(&(
        my_current_valve.to_string(),
        elephant_current_valve.to_string(),
        available_minutes,
        my_current_valve_is_not_turned_on,
        elephant_current_valve_is_not_turned_on,
    )) {
        if v >= &current_flow {
            return *v;
        }
    }

    let mut max_flow = u32::MIN;

    let available_minutes_after_move = available_minutes - 1;
    let next_steps = possible_next_steps(
        data,
        my_current_valve,
        elephant_current_valve,
        available_minutes_after_move,
        turned_on,
    );

    for (my_conf, elephant_conf) in next_steps {
        let mut turned_on = my_conf.2;
        turned_on.extend(elephant_conf.2);
        max_flow = max_flow.max(solve_with_elephant(
            data,
            &my_conf.0,
            &elephant_conf.0,
            available_minutes_after_move,
            current_flow + my_conf.1 + elephant_conf.1,
            turned_on,
            conf,
        ));
    }

    conf.insert(
        (
            my_current_valve.to_string(),
            elephant_current_valve.to_string(),
            available_minutes,
            my_current_valve_is_not_turned_on,
            elephant_current_valve_is_not_turned_on,
        ),
        current_flow,
    );

    max_flow
}

fn possible_next_steps(
    data: &HashMap<String, (u32, Vec<String>)>,
    my_current_valve: &String,
    elephant_current_valve: &String,
    available_minutes: i32,
    turned_on: HashSet<String>,
) -> Vec<(
    (String, u32, HashSet<String>),
    (String, u32, HashSet<String>),
)> {
    let (my_flow_rate, my_next_valves) = data.get(my_current_valve).unwrap();
    let mut my_configurations = vec![];
    let mut new_turned_on = turned_on.clone();
    if my_flow_rate > &0 && !turned_on.contains(my_current_valve) {
        new_turned_on.insert(my_current_valve.clone());
        my_configurations.push((
            my_current_valve.clone(),
            available_minutes as u32 * my_flow_rate,
            new_turned_on.clone(),
        ));
    }
    for next_valve in my_next_valves {
        my_configurations.push((next_valve.clone(), 0, turned_on.clone()));
    }

    let (elephant_flow_rate, elephant_next_valves) = data.get(elephant_current_valve).unwrap();
    let mut elephant_configurations = vec![];
    if elephant_flow_rate > &0
        && my_current_valve != elephant_current_valve
        && !turned_on.contains(elephant_current_valve)
    {
        let mut new_turned_on = turned_on.clone();
        new_turned_on.insert(elephant_current_valve.clone());
        elephant_configurations.push((
            elephant_current_valve.clone(),
            available_minutes as u32 * elephant_flow_rate,
            new_turned_on,
        ));
    }
    for next_valve in elephant_next_valves {
        elephant_configurations.push((next_valve.clone(), 0, turned_on.clone()));
    }

    let mut result = vec![];
    for my_conf in &my_configurations {
        for elephant_conf in &elephant_configurations {
            result.push((my_conf.clone(), elephant_conf.clone()));
        }
    }
    result
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

fn solve_part_2(file_path: &str) -> u32 {
    let data = load_file(file_path);
    let valves_with_flow_rate_and_leads_to = parse_data(data);
    let available_minutes = 26;
    let start_valve = "AA";
    solve_with_elephant(
        &valves_with_flow_rate_and_leads_to,
        &start_valve.to_string(),
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

fn part_2(file_path: &str) {
    let result = solve_part_2(file_path);
    println!("Part 2 result: {:?} (too low 2435, to high 2980)", result);
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
