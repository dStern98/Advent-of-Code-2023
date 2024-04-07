use crate::{read_input_file, SolveAdvent};
use std::collections::{HashMap, HashSet};

pub struct Day25;

impl SolveAdvent for Day25 {
    fn solve_part1(path_to_file: &str) {
        let wire_map = WireMap::from_input_file(path_to_file);
        let unique_connection_permutations = construct_all_triplets(wire_map.connections.len());
        println!(
            "Wire map has {} connections and {} triplets to remove",
            wire_map.connections.len(),
            unique_connection_permutations.len()
        );
        for (index, connections_to_remove) in unique_connection_permutations.into_iter().enumerate()
        {
            if index % 1_000 == 0 {
                println!("Checked {} potential triplets", index);
            }
            if let Some(perfect_partition) =
                try_removing_connections(&wire_map, connections_to_remove)
            {
                println!("Got a perfect partition of size {}", perfect_partition);
                break;
            }
        }
    }

    fn solve_part2(path_to_file: &str) {
        let _ = path_to_file;
    }
}

fn construct_all_triplets(connections_count: usize) -> HashSet<(usize, usize, usize)> {
    let all_connection_permutations =
        recursively_build_connection_triplets((0..connections_count).collect::<Vec<_>>(), 0);
    let unique_connection_permutations = all_connection_permutations
        .into_iter()
        .map(|mut triplet| {
            triplet.sort();
            let triplet = (triplet[0], triplet[1], triplet[2]);
            triplet
        })
        .collect::<HashSet<_>>();
    unique_connection_permutations
}

fn recursively_build_connection_triplets(
    numbers_to_choose_from: Vec<usize>,
    depth: u8,
) -> Vec<Vec<usize>> {
    if depth == 3 {
        return vec![vec![]];
    }
    let mut responses = Vec::new();
    for available_number in numbers_to_choose_from.iter() {
        let new_numbers_to_choose_from = numbers_to_choose_from
            .iter()
            .copied()
            .filter(|number| number != available_number)
            .collect::<Vec<_>>();
        let mut possible_combinations =
            recursively_build_connection_triplets(new_numbers_to_choose_from, depth + 1);
        for possible_combination in possible_combinations.iter_mut() {
            possible_combination.push(*available_number);
        }
        responses.extend(possible_combinations);
    }
    responses
}

#[derive(Debug, Clone)]

struct WireMap {
    connections: Vec<(String, String)>,
    connections_map: HashMap<String, HashSet<String>>,
}

impl WireMap {
    fn from_input_file(file_path: &str) -> Self {
        let mut connections = Vec::new();
        let mut connections_map = HashMap::new();
        let file_as_string = read_input_file(file_path);
        for line in file_as_string.lines() {
            WireMap::parse_line(&mut connections_map, &mut connections, line);
        }
        WireMap {
            connections,
            connections_map,
        }
    }

    fn parse_line(
        connections_map: &mut HashMap<String, HashSet<String>>,
        connections: &mut Vec<(String, String)>,
        line: &str,
    ) {
        let mut components = line.split(':');
        let left_component = components.next().unwrap().trim().to_string();
        let right_components = components
            .next()
            .unwrap()
            .trim()
            .split(' ')
            .map(|component| component.trim().to_string())
            .collect::<Vec<_>>();
        let component_paired = right_components
            .into_iter()
            .map(|right_component| (left_component.clone(), right_component))
            .collect::<Vec<_>>();
        connections.extend(component_paired.clone());
        for (left_component, right_component) in component_paired {
            connections_map
                .entry(left_component.clone())
                .or_default()
                .insert(right_component.clone());
            connections_map
                .entry(right_component.clone())
                .or_default()
                .insert(left_component.clone());
        }
    }
}

fn try_removing_connections(
    wire_map: &WireMap,
    connections_to_remove: (usize, usize, usize),
) -> Option<usize> {
    //! Given the 3 wires to remove from the wire map, create a copy of the `connections_map` but with
    //! those three connections removed.
    let mut connection_map_probe = wire_map.connections_map.clone();
    let (conn1, conn2, conn3) = connections_to_remove;
    let connections_to_remove = vec![
        wire_map.connections.get(conn1).unwrap(),
        wire_map.connections.get(conn2).unwrap(),
        wire_map.connections.get(conn3).unwrap(),
    ];
    //For the 3 connections chosen to be removed, remove them from the Wire Map.
    for (left_wire, right_wire) in connections_to_remove {
        connection_map_probe
            .get_mut(left_wire)
            .unwrap()
            .remove(right_wire);
        connection_map_probe
            .get_mut(right_wire)
            .unwrap()
            .remove(left_wire);
    }
    //Make sure that all keys mapped to an empty set are removed.
    connection_map_probe.retain(|_k, v| v.len() > 0);

    let mut partition_group_sizes = Vec::new();
    while !connection_map_probe.is_empty() {
        if partition_group_sizes.len() > 2 {
            //If the partition group size is greater than 2, then the
            //3 wires remove do not split the wires into 2 groups.
            return None;
        }
        let visited_wires = explore_wire(&connection_map_probe);
        partition_group_sizes.push(visited_wires.len());
        connection_map_probe.retain(|key, _value| !visited_wires.contains(key));
    }
    if partition_group_sizes.len() == 2 {
        println!(
            "Got a division that created 2 groups with contents {:?}",
            partition_group_sizes
        );
        return Some(partition_group_sizes[0] * partition_group_sizes[1]);
    }
    None
}

fn explore_wire(connection_map_probe: &HashMap<String, HashSet<String>>) -> HashSet<String> {
    //! Given the wire map (with already visited wires removed), traverse the wire until traversal
    //! is no longer possible without a cycle. Return the set of unique wires that can be traversed from the
    //! given first_connection.
    let first_connection = connection_map_probe
        .keys()
        .next()
        .expect("Connection Map was empty");

    let mut wire_stack = vec![first_connection];
    let mut visited_tracker = HashSet::new();
    while let Some(current_wire) = wire_stack.pop() {
        if !visited_tracker.contains(current_wire) {
            visited_tracker.insert(current_wire.clone());
            for next_wire in connection_map_probe.get(current_wire).unwrap() {
                wire_stack.push(next_wire);
            }
        }
    }
    visited_tracker
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_combinations() {
        // It is a fact that 33 choose 3 is 5456.
        let unique_combinations = construct_all_triplets(33);
        assert_eq!(unique_combinations.len(), 5456);
    }
}
