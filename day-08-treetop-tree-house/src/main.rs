use std::{collections::HashSet, fs};

fn load_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

fn parse_data(data: String) -> Forest {
    let trees = data
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let x_max = trees.len();
    let y_max = trees[0].len();
    Forest {
        trees,
        x_max,
        y_max,
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Forest {
    trees: Vec<Vec<u8>>,
    x_max: usize,
    y_max: usize,
}

fn count_visible_trees(forest: &Forest) -> usize {
    let mut visible_trees = HashSet::new();
    // go horizontaly
    for x in 0..forest.x_max {
        let mut max_tree_hight = -1;
        let mut y_left = 0;
        while max_tree_hight < 9 && y_left < forest.y_max {
            let tree_hight = forest.trees[x][y_left] as i8;
            if max_tree_hight < tree_hight {
                max_tree_hight = tree_hight;
                visible_trees.insert((x, y_left));
            }
            y_left += 1;
        }
        let mut max_tree_hight = -1;
        let mut y_right = forest.y_max;
        while max_tree_hight < 9 && y_right > 0 {
            y_right -= 1;
            let tree_hight = forest.trees[x][y_right] as i8;
            if max_tree_hight < tree_hight {
                max_tree_hight = tree_hight;
                visible_trees.insert((x, y_right));
            }
        }
    }

    // go verticaly
    for y in 0..forest.y_max {
        let mut max_tree_hight = -1;
        let mut x_up = 0;
        while max_tree_hight < 9 && x_up < forest.x_max {
            let tree_hight = forest.trees[x_up][y] as i8;
            if max_tree_hight < tree_hight {
                max_tree_hight = tree_hight;
                visible_trees.insert((x_up, y));
            }
            x_up += 1;
        }
        let mut max_tree_hight = -1;
        let mut x_down = forest.x_max;
        while max_tree_hight < 9 && x_down > 0 {
            x_down -= 1;
            let tree_hight = forest.trees[x_down][y] as i8;
            if max_tree_hight < tree_hight {
                max_tree_hight = tree_hight;
                visible_trees.insert((x_down, y));
            }
        }
    }
    visible_trees.len()
}

fn calculate_scenic_point(x_pos: usize, y_pos: usize, forest: &Forest) -> usize {
    let pos_tree_hight = forest.trees[x_pos][y_pos];
    let mut go = true;
    let seen_trees_x_down = ((x_pos + 1)..(forest.x_max))
        .take_while(|x| {
            let result = go;
            go = forest.trees[*x][y_pos] < pos_tree_hight;
            result
        })
        .count();
    let mut go = true;
    let seen_trees_x_up = (0..x_pos)
        .rev()
        .take_while(|x| {
            let result = go;
            go = forest.trees[*x][y_pos] < pos_tree_hight;
            result
        })
        .count();
    let mut go = true;
    let seen_trees_y_right = ((y_pos + 1)..(forest.y_max))
        .take_while(|y| {
            let result = go;
            go = forest.trees[x_pos][*y] < pos_tree_hight;
            result
        })
        .count();
    let mut go = true;
    let seen_trees_y_left = (0..y_pos)
        .rev()
        .take_while(|y| {
            let result = go;
            go = forest.trees[x_pos][*y] < pos_tree_hight;
            result
        })
        .count();
    seen_trees_x_down * seen_trees_x_up * seen_trees_y_right * seen_trees_y_left
}

fn find_the_best_scenic_point(forest: &Forest) -> usize {
    let mut score = 0;
    for x in 0..forest.x_max {
        for y in 0..forest.y_max {
            score = score.max(calculate_scenic_point(x, y, forest));
        }
    }
    score
}

fn solve_part_1(file_path: &str) -> usize {
    let data = load_file(file_path);
    let forest = parse_data(data);
    count_visible_trees(&forest)
}

fn solve_part_2(file_path: &str) -> usize {
    let data = load_file(file_path);
    let forest = parse_data(data);
    find_the_best_scenic_point(&forest)
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
        assert_eq!(result, 21);
    }

    #[test]
    fn test_part_2() {
        let result = solve_part_2("./resources/test_data.txt");
        assert_eq!(result, 8);
    }

    #[test]
    fn test_score_scenic_1_2() {
        let data = load_file("./resources/test_data.txt");
        let forest = parse_data(data);
        assert_eq!(calculate_scenic_point(1, 2, &forest), 4);
    }

    #[test]
    fn test_score_scenic_3_2() {
        let data = load_file("./resources/test_data.txt");
        let forest = parse_data(data);
        assert_eq!(calculate_scenic_point(3, 2, &forest), 8);
    }
}
