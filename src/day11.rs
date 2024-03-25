use crate::{read_input_file, SolveAdvent};
use std::collections::{HashSet, VecDeque};

pub struct Day11;

impl SolveAdvent for Day11 {
    fn solve_part1(path_to_file: &str) {
        let file_as_str = read_input_file(path_to_file);
        let universe = Universe::new(&file_as_str, 2);
        universe.find_minimum_distance_between_all_galaxies();
    }

    fn solve_part2(path_to_file: &str) {
        let file_as_str = read_input_file(path_to_file);
        let universe = Universe::new(&file_as_str, 1_000_000);
        universe.find_minimum_distance_between_all_galaxies();
    }
}

///A Galaxy is a '#' on the input map.
#[derive(Debug, Clone, Copy)]
struct Galaxy {
    ///Unique ID for the galaxy.
    id: usize,
    row: usize,
    col: usize,
}

impl Galaxy {
    fn identify_galaxies(universe_map: &[Vec<char>]) -> Vec<Galaxy> {
        //! Process the Universe, taking note of the position of all Galaxies.
        let mut identified_galaxies = Vec::new();
        for (row_number, row) in universe_map.iter().enumerate() {
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
}

///Represents the Universe of the problem.
#[derive(Debug, Clone)]
struct Universe {
    ///The column numbers that are expanded because
    /// they have only '.'
    expanded_columns: HashSet<usize>,
    ///The row numbers that are expanded because they have only '.'
    expanded_rows: HashSet<usize>,
    ///A nod to cosmology. How much each expanded row or column is expanded by.
    scale_factor: usize,
    /// All of the galaxies in this Universe.
    galaxies: Vec<Galaxy>,
    /// The dimensions of the universe
    dimensions: (usize, usize),
}

impl Universe {
    fn new(file_as_str: &str, scale_factor: usize) -> Universe {
        //! Constructs a new Universe.
        let galaxies_disassembled = file_as_str
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        //Take note of the dimensions of the universe
        let universe_dimensions = (galaxies_disassembled.len(), galaxies_disassembled[0].len());

        //Take note of the row numbers that are expanded
        let mut expanded_rows = HashSet::new();
        for (row_number, row_content) in galaxies_disassembled.iter().enumerate() {
            if row_content.iter().all(|char| char == &'.') {
                expanded_rows.insert(row_number);
            }
        }

        //Take note of the column numbers that are expanded.
        let mut expanded_columns = HashSet::new();
        let mut column_number = 0;
        while column_number < galaxies_disassembled[0].len() {
            let mut column_content = galaxies_disassembled
                .iter()
                .map(|row| row.get(column_number).unwrap());
            if column_content.all(|char| char == &'.') {
                expanded_columns.insert(column_number);
            }
            column_number += 1;
        }
        //Locate all galaxies in the universe.
        let galaxies = Galaxy::identify_galaxies(&galaxies_disassembled);

        println!(
            "------------There are {} total galaxies in this universe with scale factor: {}-------------",
            galaxies.len(), 
            scale_factor
        );

        Universe {
            expanded_columns,
            expanded_rows,
            scale_factor,
            dimensions: universe_dimensions,
            galaxies,
        }
    }

    fn find_minimum_distance_between_all_galaxies(&self) {
        //! Perform the actual computation required of Day11. Find the sum of the shortest distance
        //! between each unique pair of galaxies.
        let galaxy_pairs = self.unique_galaxy_pairs();
        println!("Found {} unique galaxy pairs", galaxy_pairs.len());
        let mut total_minimum_distance = 0;
        for (galaxy_1_id, galaxy_2_id) in galaxy_pairs.into_iter() {
            let galaxy_1 = self.galaxies.get(galaxy_1_id).unwrap();
            let galaxy_2 = self.galaxies.get(galaxy_2_id).unwrap();
            let minimum_distance = find_minimum_distance(galaxy_1, galaxy_2, self);
            total_minimum_distance += minimum_distance;
        }
        println!(
            "Total minimum distances between galaxies is: {}",
            total_minimum_distance
        );
    }

    fn unique_galaxy_pairs(&self) -> HashSet<(usize, usize)> {
        //! Determine the minimum set of unique galaxy pairs that must be investigated, as order does
        //! not matter.
        //! The easiest way to do this is to simply iterate twice through the same array, and
        //! only log pairs when the id of galaxy_1 is less than the id of galaxy_2.
        let mut seen_pairs = HashSet::new();
        for galaxy1 in self.galaxies.iter() {
            for galaxy_2 in self.galaxies.iter() {
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
}

///A probe is trying to roam the Universe
/// and find the target.
#[derive(Debug, Clone, Copy)]
struct Probe<'a> {
    row: usize,
    col: usize,
    distance_traveled: usize,
    target_row: usize,
    target_col: usize,
    universe: &'a Universe,
}

impl<'a> Probe<'a> {
    fn target_found(&self) -> bool {
        //! Has the probe found the target?
        self.col == self.target_col && self.row == self.target_row
    }
    fn distance_to_target(&self) -> f64 {
        //! Apply the straightforward definition of distance to determine how far
        //! away the probe is from the target. This is an essential optimization to allow
        //! for each probe to only move in the directions the minimizes the distance.
        //!
        //! It is a non-trivial fact to note that the distance without taking into
        //! account the expansion of the universe can be used here. The shortest path from
        //! one galaxy to another is still the same regardless of the universe scaling, only the distance
        //! traveled is variable based on scale factor. THIS WOULD NOT BE TRUE IF THE EXPANSION WERE IRREGULAR!
        let distance_x = self.target_row as f64 - self.row as f64;
        let distance_y = self.target_col as f64 - self.col as f64;
        ((distance_x).powf(2.0) + (distance_y).powf(2.0)).sqrt()
    }
    fn left_one(&self) -> Option<Probe<'a>> {
        //! Attempt to get a new probe the moves left. This will return None
        //! if going left takes the probe outside of the bounds of the Universe or if moving
        //! left increases the distance to the target.
        if self.col == 0 {
            return None;
        }
        //If the probe is crossing an `expanded` column, then the distance traveled
        //needs to be increased by the scale factor instead of 1.
        let distance_this_step = if self.universe.expanded_columns.contains(&self.col) {
            self.universe.scale_factor
        } else {
            1
        };
        let left_one = Probe {
            row: self.row,
            col: self.col - 1,
            distance_traveled: self.distance_traveled + distance_this_step,
            target_col: self.target_col,
            target_row: self.target_row,
            universe: self.universe,
        };
        if left_one.distance_to_target() > self.distance_to_target() {
            return None;
        }
        Some(left_one)
    }

    fn right_one(&self) -> Option<Probe<'a>> {
        //! Attempt to get a probe one to the right.
        let universe_column_boundary = self.universe.dimensions.1;
        if self.col + 1 >= universe_column_boundary {
            return None;
        }
        //If the probe is crossing an `expanded` column, then the distance traveled
        //needs to be increased by the scale factor instead of 1.
        let distance_this_step = if self.universe.expanded_columns.contains(&self.col) {
            self.universe.scale_factor
        } else {
            1
        };
        let right_one = Probe {
            row: self.row,
            col: self.col + 1,
            distance_traveled: self.distance_traveled + distance_this_step,
            target_col: self.target_col,
            target_row: self.target_row,
            universe: self.universe,
        };
        if right_one.distance_to_target() > self.distance_to_target() {
            return None;
        }
        Some(right_one)
    }

    fn up_one(&self) -> Option<Probe<'a>> {
        //! Attempt to get a probe one up, which may fail.
        if self.row == 0 {
            return None;
        }
        //If the current probe is crossing an expanded row, then the distance traveled
        //in this step is the scale factor rather than 1.
        let distance_this_step = if self.universe.expanded_rows.contains(&self.row) {
            self.universe.scale_factor
        } else {
            1
        };
        let up_one = Probe {
            row: self.row - 1,
            col: self.col,
            distance_traveled: self.distance_traveled + distance_this_step,
            target_col: self.target_col,
            target_row: self.target_row,
            universe: self.universe,
        };
        if up_one.distance_to_target() > self.distance_to_target() {
            return None;
        }
        Some(up_one)
    }

    fn down_one(&self) -> Option<Probe<'a>> {
        //! Attempt to get a probe one down. This may fail for various reasons.
        let row_boundary = self.universe.dimensions.0;
        if self.row + 1 >= row_boundary {
            return None;
        }
        //If the current probe is crossing an expanded row, then the distance traveled
        //in this step is the scale factor rather than 1.
        let distance_this_step = if self.universe.expanded_rows.contains(&self.row) {
            self.universe.scale_factor
        } else {
            1
        };
        let down_one = Probe {
            row: self.row + 1,
            col: self.col,
            distance_traveled: self.distance_traveled + distance_this_step,
            target_col: self.target_col,
            target_row: self.target_row,
            universe: self.universe,
        };
        if down_one.distance_to_target() > self.distance_to_target() {
            return None;
        }
        Some(down_one)
    }
}

fn find_minimum_distance(galaxy_1: &Galaxy, galaxy_2: &Galaxy, universe: &Universe) -> usize {
    //! Use breadth first search to traverse the universe, using probes. Breadth first search is used
    //! so that as soon as a single probe finds the target, the distance traveled by the probe is guranteed to be
    //! the minimum distance.
    //!
    //! To optimize the performance, of the 4 possible probe movement directions, only the direction
    //! that moves the probe closest to the target is tested. Because of this optimization, we do not need
    //! a hashset to protect against cycles. The probe traversal will never get stuck in a cycle because each iteration
    //! it only moves in the optimal direction.

    //Use a VecDeque for efficient breadth first search.
    let mut probes_tracker = VecDeque::new();

    //The first probe starts at Galaxy_1, with Galaxy_2 as its target.
    probes_tracker.push_back(Probe {
        row: galaxy_1.row,
        col: galaxy_1.col,
        distance_traveled: 0,
        target_col: galaxy_2.col,
        target_row: galaxy_2.row,
        universe,
    });

    while !probes_tracker.is_empty() {
        let latest_probe = probes_tracker.pop_front().unwrap();
        if latest_probe.target_found() {
            return latest_probe.distance_traveled;
        }

        let mut next_probes = [
            latest_probe.left_one(),
            latest_probe.right_one(),
            latest_probe.up_one(),
            latest_probe.down_one(),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();
        next_probes.sort_by(|a, b| {
            a.distance_to_target()
                .partial_cmp(&b.distance_to_target())
                .unwrap()
        });
        let next_probe = next_probes[0];
        probes_tracker.push_back(next_probe);
    }

    panic!("No shortest path was found between galaxies, which is impossible!");
}
