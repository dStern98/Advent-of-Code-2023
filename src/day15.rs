use crate::{read_input_file, SolveAdvent};
use std::collections::HashMap;

pub struct Day15;

impl SolveAdvent for Day15 {
    fn solve_part1(path_to_file: &str) {
        let file_as_str = read_input_file(path_to_file);
        let init_sequence = file_as_str.split(',');
        let mut total_hash_value = 0;
        for sequence in init_sequence {
            total_hash_value += hash(sequence);
        }
        println!("Sum of hashes of each sequence step: {}", total_hash_value);
    }

    fn solve_part2(path_to_file: &str) {
        let file_as_str = read_input_file(path_to_file);
        let init_sequence = file_as_str.split(',');

        //Intialize the lens_map to contain all empty boxes from 0..255
        let mut lens_map = HashMap::with_capacity(256);
        for i in 0..256 {
            lens_map.insert(i, Vec::new());
        }

        for step in init_sequence {
            if step.contains('=') {
                handle_equals_op(step, &mut lens_map);
            } else if step.contains('-') {
                handle_dash_op(step, &mut lens_map);
            }
        }

        let mut total_focusing_power = 0;
        for (box_number, lens_box) in lens_map {
            for (slot_number, (_lens_label, lens_power)) in lens_box.into_iter().enumerate() {
                total_focusing_power += (1 + box_number) * (slot_number + 1) * lens_power;
            }
        }
        println!("Total Focusing Power: {}", total_focusing_power);
    }
}

fn hash(input: &str) -> usize {
    //! Perform the hash algorithm as describe in the challenge.
    let mut hash_value = 0;
    let input_as_bytes = input.as_bytes();
    for ascii_bytes in input_as_bytes {
        hash_value += *ascii_bytes as usize;
        hash_value *= 17;
        hash_value %= 256;
    }
    hash_value
}

fn handle_equals_op(step: &str, lens_map: &mut HashMap<usize, Vec<(String, usize)>>) {
    //! Perform the changes to the lens_map as described for the equals operations.
    let step = step.trim();
    let mut step_splitter = step.split('=');
    let label_to_insert = step_splitter.next().unwrap();
    let focal_length_to_insert = step_splitter.next().unwrap().parse::<usize>().unwrap();
    let specified_box = lens_map.get_mut(&hash(label_to_insert)).unwrap();

    for (label, focal_size) in specified_box.iter_mut() {
        if label == label_to_insert {
            //As described in the directions, if the label is in the box,
            //set the focal_size to the new focal_size to insert according to the step
            *focal_size = focal_length_to_insert;
            return;
        }
    }
    //If the lens is not in the box, then insert at the end of the box.
    specified_box.push((label_to_insert.to_owned(), focal_length_to_insert));
}

fn handle_dash_op(step: &str, lens_map: &mut HashMap<usize, Vec<(String, usize)>>) {
    //! Perform the changes to the lens_map as described for a dash operation.
    let step = step.trim();
    let label = step.replace('-', "");
    let specified_box = lens_map.get_mut(&hash(&label)).unwrap();

    let mut index = 0;
    while index < specified_box.len() {
        let (box_label, _) = specified_box.get(index).unwrap();
        if box_label == &label {
            //If the lens is in the box, remove it from the box.
            specified_box.remove(index);
            break;
        }
        index += 1;
    }
}
