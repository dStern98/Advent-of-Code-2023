use crate::{read_input_file, SolveAdvent};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub struct Day25;

impl SolveAdvent for Day25 {
    fn solve_part1(path_to_file: &str) {
        let wire_map = WireMap::from_input_file(path_to_file);
        let unique_connection_permutations = (0..wire_map.connections.len()).combinations(3);

        println!(
            "Wire map has {} connections and {} wire triplets to test",
            wire_map.connections.len(),
            unique_connection_permutations.clone().count()
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
    connections_to_remove: Vec<usize>,
) -> Option<usize> {
    //! Given the 3 wires to remove from the wire map, create a copy of the `connections_map` but with
    //! those three connections removed.
    let mut connection_map_probe = wire_map.connections_map.clone();
    let connections_to_remove = connections_to_remove
        .into_iter()
        .map(|connection_to_remove| wire_map.connections.get(connection_to_remove).unwrap())
        .collect::<Vec<_>>();
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
    connection_map_probe.retain(|_k, v| !v.is_empty());

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
