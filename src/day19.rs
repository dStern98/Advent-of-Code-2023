use crate::{read_file_to_string, SolveAdvent};
use std::collections::HashMap;

pub struct Day19;

impl SolveAdvent for Day19 {
    fn solve_part1(path_to_file: &str) {
        let file_as_str = read_file_to_string(path_to_file);
        let (part_ratings, workflows) = process_into_workflows_ratings(file_as_str);
        let mut total_ratings_number = 0;
        for part in part_ratings {
            let mut current_workflow = workflows.get("in").unwrap();
            loop {
                let next_workflow = find_next_workflow(&part, current_workflow);
                if next_workflow == "A".to_owned() {
                    let part_score_total: usize = part.values().map(|item| item.clone()).sum();
                    total_ratings_number += part_score_total;
                    break;
                } else if next_workflow == "R".to_owned() {
                    break;
                } else {
                    current_workflow = workflows.get(&next_workflow).unwrap();
                }
            }
        }
        println!(
            "Total Accepted Part Ratings Numbers: {}",
            total_ratings_number
        );
    }

    fn solve_part2(path_to_file: &str) {
        let file_as_str = read_file_to_string(path_to_file);
        let (_, workflows) = process_into_workflows_ratings(file_as_str);
        //Walk all possible workflows
        let mut parsed_workflows = traverse_workflows_recursively(&"in".to_owned(), &workflows);
        //Because workflows are returned in the opposite order that they are traversed, we reverse all of the traversals.
        for workflow in parsed_workflows.iter_mut() {
            workflow.reverse();
        }
        let total_acceptable_paths: usize = parsed_workflows
            .into_iter()
            .filter(|workflow| workflow.iter().last().unwrap() == &"A".to_owned())
            .map(|acceptable_path| count_valid_rating_combinations(acceptable_path))
            .sum();

        println!(
            "Total number of unique acceptable ratings: {}",
            total_acceptable_paths
        );
    }
}

fn find_next_workflow(rating: &HashMap<String, usize>, workflow_rules: &Vec<String>) -> String {
    //! Given a rating and a workflow's rules, return the destination (either A for accepted, R for Rejected, or
    //! the key to another workflow).
    for rule in workflow_rules.iter() {
        if !rule.contains(':') {
            //If no colon is present, then the rule is itself a destination,
            //so simply return it.
            return rule.to_owned();
        }
        let mut rule_split = rule.split(':');
        let rule_actual = rule_split.next().unwrap();
        let destination = rule_split.next().unwrap();
        let rule_symbol = if rule_actual.contains('>') { ">" } else { "<" };
        let mut rule_actual_split = rule_actual.split(rule_symbol);
        //Obtain the rule_key, rule_value to test against
        //(for example x<256 means x is the key_to_test and 256 is the number_to_test)
        let key_to_test = rule_actual_split.next().unwrap();
        let number_to_test = rule_actual_split.next().unwrap().parse::<usize>().unwrap();
        let rated_value = rating.get(key_to_test).unwrap();
        //If the rated_value matches a logical rule, return that destination.
        if rule_symbol == ">" && rated_value > &number_to_test {
            return destination.to_owned();
        } else if rule_symbol == "<" && rated_value < &number_to_test {
            return destination.to_owned();
        }
    }
    panic!("Last rule contained a colon");
}

fn process_into_workflows_ratings(
    file_as_str: String,
) -> (Vec<HashMap<String, usize>>, HashMap<String, Vec<String>>) {
    //! Given the input file as a String, process the ratings and workflows.
    let mut ratings = Vec::new();
    let mut workflows = Vec::new();

    for line in file_as_str.lines() {
        //We know that all ratings start with a {
        if line.starts_with("{") {
            ratings.push(line)
        //Ignore any empty lines.
        } else if line.len() > 0 {
            workflows.push(line);
        }
    }
    let ratings =
        ratings
            .into_iter()
            .map(|item| {
                HashMap::from_iter(item.replace("{", "").replace("}", "").split(',').map(
                    |rating| {
                        let mut rating = rating.split('=');
                        let key = rating.next().unwrap().to_owned();
                        let value = rating.next().unwrap().parse::<usize>().unwrap();
                        (key, value)
                    },
                ))
            })
            .collect::<Vec<_>>();

    let mut workflow_map = HashMap::with_capacity(workflows.len());
    for workflow in workflows {
        let workflow = workflow.replace("}", "");
        let mut workflow_split = workflow.split('{');
        let workflow_name = workflow_split.next().unwrap().to_owned();
        let workflow_rules = workflow_split
            .next()
            .unwrap()
            .split(',')
            .map(|item| item.to_owned())
            .collect::<Vec<_>>();
        workflow_map.insert(workflow_name, workflow_rules);
    }
    (ratings, workflow_map)
}

fn traverse_workflows_recursively(
    current_map_state: &String,
    workflows: &HashMap<String, Vec<String>>,
) -> Vec<Vec<String>> {
    //! Recursively walk the tree of workflows, returning when the product is either rejected (R)
    //! or accepted(A). Along the way, all of the logic branch that lead to each end state are recorded
    //! in a vector, so that at the return of the first function invocation, all possible logical branch
    //! have been completely populated.
    if current_map_state == &"A".to_owned() {
        return vec![vec!["A".to_owned()]];
    } else if current_map_state == &"R".to_owned() {
        return vec![vec!["R".to_owned()]];
    } else {
        //If none of the base cases is triggered, then recursively take each logical branch down the tree.
        let this_workflows_directions = workflows.get(current_map_state).unwrap();
        let mut outer_vec = Vec::new();
        //Get the Inverses of each of the rules in this workflow.
        let rule_inverses = generate_inverse_of_rules(this_workflows_directions);
        for (index, direction) in this_workflows_directions.into_iter().enumerate() {
            if direction.contains(':') {
                //An example direction is s>2770:qs
                let this_directions_inverses = rule_inverses
                    .clone()
                    .drain(0..index)
                    .rev()
                    .collect::<Vec<_>>();
                let mut current_workflow_direction_split = direction.split(':');
                //In this example workflow rule is s>2770
                let workflow_rule = current_workflow_direction_split.next().unwrap();
                //In this example, go_to_node is qs
                let go_to_node = current_workflow_direction_split.next().unwrap();
                let mut next_nodes_response =
                    traverse_workflows_recursively(&go_to_node.to_string(), workflows);
                //After the recursive call returns, add the workflow_rule that triggered the recursive call,
                //as well as all of the applicable inverses.
                for node in next_nodes_response.iter_mut() {
                    node.push(workflow_rule.to_owned());
                    node.extend(this_directions_inverses.clone());
                }
                outer_vec.extend(next_nodes_response);
            } else {
                //Example direction: rfg. Here the go_to_node is the direction, so no direction parsing is necessary.
                let mut next_nodes_response = traverse_workflows_recursively(direction, workflows);
                for node in next_nodes_response.iter_mut() {
                    //Because only the last clause in a workflow has no conditional, we know that
                    // the inverse of all of the previous rules in this workflow must all be true to have
                    //reached this point.
                    let mut rules_inverse = rule_inverses.clone();
                    rules_inverse.reverse();
                    node.extend(rules_inverse);
                }
                outer_vec.extend(next_nodes_response);
            }
        }
        return outer_vec;
    }
}

fn generate_inverse_of_rules(rules: &Vec<String>) -> Vec<String> {
    //! Given the rules of certain workflow, product the inverse rules (
    //! the opposite of each rule).
    rules
        .iter()
        .filter_map(|rule| {
            if rule.contains(':') {
                let mut rule_split = rule.split(':');
                let rule_component = rule_split.next().unwrap();
                return Some(format!("!{}", rule_component));
            }
            None
        })
        .collect::<Vec<_>>()
}

fn count_valid_rating_combinations(mut acceptable_branch: Vec<String>) -> usize {
    //! Given an acceptable branch (a valid set of comparisons that reach A), return the unique
    //! number of ratings that satisfy the branch requirements.
    //First, remove the "A" which we know must be at the end of the branch.
    acceptable_branch.pop().unwrap();
    //Generate a HashMap with each rating type mapped to all 4000 possible rating values.
    let rating_range = (1..4001).collect::<Vec<u64>>();
    let mut allowed_combinations = HashMap::from([
        ("s".to_owned(), rating_range.clone()),
        ("a".to_owned(), rating_range.clone()),
        ("x".to_owned(), rating_range.clone()),
        ("m".to_owned(), rating_range),
    ]);

    //Iterate over the rules in the acceptable branch
    for rule in acceptable_branch {
        //Parse the rule into a logical operation, a number, and a key.
        let negate_truth_test = if rule.contains('!') { true } else { false };
        let comparison_operation = if rule.contains('>') { ">" } else { "<" };
        let rule = rule.replace("!", "");
        let mut rule_iterator = rule.split(comparison_operation);
        let rule_key = rule_iterator.next().unwrap().to_owned();
        let rule_value = rule_iterator.next().unwrap().parse::<u64>().unwrap();
        let rating_combinations_available = allowed_combinations.get_mut(&rule_key).unwrap();
        //Filter out of that rating any value that the workflow would not accept.
        let mut index = 0;
        while index < rating_combinations_available.len() {
            let rating = rating_combinations_available.get(index).unwrap();
            let truth_test = if comparison_operation == ">" && negate_truth_test {
                !(rating <= &rule_value)
            } else if comparison_operation == ">" && !negate_truth_test {
                rating <= &rule_value
            } else if comparison_operation == "<" && negate_truth_test {
                !(rating >= &rule_value)
            } else {
                rating >= &rule_value
            };
            if truth_test {
                rating_combinations_available.remove(index);
            } else {
                index += 1;
            }
        }
    }
    //Multiply the number of values in each of the 4 rating types to get the number of unique
    //combinations.
    allowed_combinations
        .values()
        .map(|acceptable_range| acceptable_range.len())
        .product::<usize>()
}
