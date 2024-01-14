use crate::{read_file_to_string, SolveAdvent};
use std::collections::HashMap;

pub struct Day8;

impl SolveAdvent for Day8 {
    fn solve_part1(path_to_file: &str) {
        let file_as_str = read_file_to_string(path_to_file);
        let (lr_pattern, lr_map) = process_input_file(file_as_str);

        let mut current_position = "AAA".to_owned();
        let mut steps = 0;
        for step in lr_pattern.chars().cycle() {
            if &current_position == "ZZZ" {
                break;
            }
            steps += 1;
            let (left_path, right_path) = lr_map.get(&current_position).unwrap();
            if step == 'L' {
                current_position = left_path.to_owned();
            } else {
                current_position = right_path.to_owned();
            }
        }
        println!("Reached ZZZ in {} steps", steps);
    }

    fn solve_part2(path_to_file: &str) {
        let file_as_str = read_file_to_string(path_to_file);
        let (lr_pattern, lr_map) = process_input_file(file_as_str);
        //Start at all nodes the end with A
        let starting_nodes = lr_map
            .keys()
            .filter(|position| position.ends_with("A"))
            .collect::<Vec<_>>();
        let mut steps_to_reach_ending_in_z = Vec::new();

        //Determine how many steps it takes starting at each A node to reach a node that ends in Z.
        for starting_node in starting_nodes {
            let mut current_position = starting_node.to_owned();
            let mut steps = 0;
            for step in lr_pattern.chars().cycle() {
                if current_position.ends_with("Z") {
                    break;
                }
                steps += 1;
                let (left_path, right_path) = lr_map.get(&current_position).unwrap();
                if step == 'L' {
                    current_position = left_path.to_owned();
                } else {
                    current_position = right_path.to_owned();
                }
            }
            steps_to_reach_ending_in_z.push(steps);
        }
        println!(
            "Steps for each node to reach ending in Z: {:?}",
            steps_to_reach_ending_in_z
        );
        //The key insight is to realize that the LCM of the vec of steps
        //will be the first time that all ghosts are in a location ending with z.
        //Just google the LCM of the numbers:
        //https://www.calculator.net/lcm-calculator.html
    }
}

fn process_input_file(file_as_str: String) -> (String, HashMap<String, (String, String)>) {
    let mut line_iterator = file_as_str.lines();
    //Extract the left-right pattern
    let left_right_pattern = line_iterator.next().unwrap().to_owned();
    //Skip the empty line
    line_iterator.next().unwrap();

    //Process the rest into a HashMap
    let mut left_right_map = HashMap::new();
    for line in line_iterator {
        let mut equals_splitter = line.split('=');
        let key = equals_splitter.next().unwrap().trim().to_owned();
        let value = equals_splitter
            .next()
            .unwrap()
            .trim()
            .replace("(", "")
            .replace(")", "");
        let mut value_iterator = value.split(',').map(|item| item.trim());
        let value1 = value_iterator.next().unwrap().to_owned();
        let value2 = value_iterator.next().unwrap().to_owned();
        left_right_map.insert(key, (value1, value2));
    }

    (left_right_pattern, left_right_map)
}
