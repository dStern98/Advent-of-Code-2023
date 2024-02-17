use crate::{read_file_to_string, SolveAdvent};
use std::collections::{HashSet, VecDeque};

pub struct Day11;

#[derive(Debug, Clone, Copy)]
struct Galaxy {
    id: usize,
    row: usize,
    col: usize,
}

#[derive(Debug, Clone, Copy)]
struct Probe {
    row: usize,
    col: usize,
    distance_traveled: usize,
    target_row: usize,
    target_col: usize,
}

impl Probe {
    fn target_found(&self) -> bool {
        self.col == self.target_col && self.row == self.target_row
    }
    fn distance_to_target(&self) -> f64 {
        let distance_x = self.target_row as f64 - self.row as f64;
        let distance_y = self.target_col as f64 - self.col as f64;
        let distance = (distance_x).powf(2.0) + (distance_y).powf(2.0);
        distance.sqrt()
    }
    fn left_one(&self, universe_dimensions: (usize, usize)) -> Option<Probe> {
        let _ = universe_dimensions;
        if self.col == 0 {
            return None;
        }
        let left_one = Probe {
            row: self.row,
            col: self.col - 1,
            distance_traveled: self.distance_traveled + 1,
            target_col: self.target_col,
            target_row: self.target_row,
        };
        if left_one.distance_to_target() > self.distance_to_target() {
            return None;
        }
        Some(left_one)
    }

    fn right_one(&self, universe_dimensions: (usize, usize)) -> Option<Probe> {
        let (_, column_boundary) = universe_dimensions;
        if self.col + 1 >= column_boundary {
            return None;
        }
        let right_one = Probe {
            row: self.row,
            col: self.col + 1,
            distance_traveled: self.distance_traveled + 1,
            target_col: self.target_col,
            target_row: self.target_row,
        };
        if right_one.distance_to_target() > self.distance_to_target() {
            return None;
        }
        Some(right_one)
    }

    fn up_one(&self, universe_dimensions: (usize, usize)) -> Option<Probe> {
        let _ = universe_dimensions;
        if self.row == 0 {
            return None;
        }
        let up_one = Probe {
            row: self.row - 1,
            col: self.col,
            distance_traveled: self.distance_traveled + 1,
            target_col: self.target_col,
            target_row: self.target_row,
        };
        if up_one.distance_to_target() > self.distance_to_target() {
            return None;
        }
        Some(up_one)
    }

    fn down_one(&self, universe_dimensions: (usize, usize)) -> Option<Probe> {
        let (row_boundary, _) = universe_dimensions;
        if self.row + 1 >= row_boundary {
            return None;
        }
        let down_one = Probe {
            row: self.row + 1,
            col: self.col,
            distance_traveled: self.distance_traveled + 1,
            target_col: self.target_col,
            target_row: self.target_row,
        };
        if down_one.distance_to_target() > self.distance_to_target() {
            return None;
        }
        Some(down_one)
    }
}

impl SolveAdvent for Day11 {
    fn solve_part1(path_to_file: &str) {
        let file_as_str = read_file_to_string(path_to_file);
        let expanded_universe = expand_universe(&file_as_str);
        let identifed_galaxies = identify_galaxies(&expanded_universe);
        println!(
            "There are {} total galaxies in this universe.",
            identifed_galaxies.len()
        );
        let galaxy_pairs = unique_galaxy_pairs(&identifed_galaxies);
        println!("Found {} unique galaxy pairs", galaxy_pairs.len());
        let universe_dimensions = (expanded_universe.len(), expanded_universe[0].len());
        let mut total_minimum_distance = 0;
        for (galaxy_pair_id, (galaxy_1_id, galaxy_2_id)) in galaxy_pairs.into_iter().enumerate() {
            let galaxy_1 = identifed_galaxies.get(galaxy_1_id).unwrap();
            let galaxy_2 = identifed_galaxies.get(galaxy_2_id).unwrap();
            let minimum_distance = find_minimum_distance(galaxy_1, galaxy_2, universe_dimensions);
            total_minimum_distance += minimum_distance;
            if galaxy_pair_id % 1000 == 0 {
                println!("Finished processing {} galaxy pairs", galaxy_pair_id)
            }
        }
        println!("Total Minimum Distances sum to {}", total_minimum_distance);
    }

    fn solve_part2(path_to_file: &str) {
        let _ = path_to_file;
    }
}

fn expand_universe(file_as_str: &str) -> Vec<Vec<char>> {
    let mut galaxies_disassembled = file_as_str
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut row_number = 0;
    while row_number < galaxies_disassembled.len() {
        let row_count = galaxies_disassembled.len();
        let row_content = galaxies_disassembled.get(row_number).unwrap();
        if row_content.iter().all(|char| char == &'.') {
            galaxies_disassembled
                .insert(row_number, (0..row_count).map(|_| '.').collect::<Vec<_>>());
            row_number += 2;
        } else {
            row_number += 1;
        }
    }

    let mut column_number = 0;
    while column_number < galaxies_disassembled[0].len() {
        let column_content = galaxies_disassembled
            .iter()
            .map(|row| row.get(column_number).unwrap())
            .collect::<Vec<_>>();
        if column_content.iter().all(|char| char == &&'.') {
            for row in galaxies_disassembled.iter_mut() {
                row.insert(column_number, '.');
            }
            column_number += 2;
        } else {
            column_number += 1;
        }
    }

    galaxies_disassembled
}

fn identify_galaxies(expanded_universe: &Vec<Vec<char>>) -> Vec<Galaxy> {
    let mut identified_galaxies = Vec::new();
    for (row_number, row) in expanded_universe.iter().enumerate() {
        for (column_number, col_value) in row.iter().enumerate() {
            if col_value == &'#' {
                let galaxy_number = identified_galaxies.len();
                identified_galaxies.push(Galaxy {
                    id: galaxy_number,
                    row: row_number,
                    col: column_number,
                });
            }
        }
    }
    identified_galaxies
}

fn unique_galaxy_pairs(identified_galaxies: &Vec<Galaxy>) -> HashSet<(usize, usize)> {
    let mut seen_pairs = HashSet::new();
    for galaxy1 in identified_galaxies.iter() {
        for galaxy_2 in identified_galaxies.iter() {
            if galaxy1.id < galaxy_2.id {
                seen_pairs.insert((galaxy1.id, galaxy_2.id));
                if seen_pairs.contains(&(galaxy_2.id, galaxy1.id)) {
                    panic!(
                        "Pair {:?} is in the seen pairs as we insert the reverse!",
                        (galaxy_2.id, galaxy1.id)
                    );
                }
            }
        }
    }
    seen_pairs
}

fn find_minimum_distance(
    galaxy_1: &Galaxy,
    galaxy_2: &Galaxy,
    universe_dimensions: (usize, usize),
) -> usize {
    let mut probes_tracker = VecDeque::new();
    probes_tracker.push_back(Probe {
        row: galaxy_1.row,
        col: galaxy_1.col,
        distance_traveled: 0,
        target_col: galaxy_2.col,
        target_row: galaxy_2.row,
    });
    let mut already_traveled = HashSet::new();
    while !probes_tracker.is_empty() {
        let latest_probe = probes_tracker.pop_front().unwrap();
        if latest_probe.target_found() {
            return latest_probe.distance_traveled;
        }
        if already_traveled.contains(&(latest_probe.row, latest_probe.col)) {
            continue;
        }
        already_traveled.insert((latest_probe.row, latest_probe.col));

        let mut next_probes = [
            latest_probe.left_one(universe_dimensions),
            latest_probe.right_one(universe_dimensions),
            latest_probe.up_one(universe_dimensions),
            latest_probe.down_one(universe_dimensions),
        ]
        .into_iter()
        .filter_map(|next_probe| next_probe)
        .collect::<Vec<_>>();
        next_probes.sort_by(|a, b| {
            a.distance_to_target()
                .partial_cmp(&b.distance_to_target())
                .unwrap()
        });
        let next_probe = next_probes[0];
        probes_tracker.push_back(next_probe);
    }

    panic!("No shortest path was found between galaxies, which is impossible");
}
