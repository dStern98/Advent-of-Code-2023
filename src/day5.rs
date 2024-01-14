use crate::{read_file_to_string, SolveAdvent};
use std::collections::HashMap;

pub struct Day5;

impl SolveAdvent for Day5 {
    fn solve_part1(path_to_file: &str) {
        let (seeds, maps_hashmap) = process_into_maps(path_to_file);
        let processed_maps = process_maps_into_numbers(maps_hashmap);
        let mut min_seen = usize::MAX;
        for seed_input in seeds {
            let location_value = apply_chain(seed_input, &processed_maps);
            if location_value < min_seen {
                min_seen = location_value;
            }
        }

        println!("Minimum location value is {}", min_seen);
    }

    fn solve_part2(path_to_file: &str) {
        let _ = path_to_file;
        todo!();
    }
}

fn process_into_maps(fp: &str) -> (Vec<usize>, HashMap<String, String>) {
    //! Given the input fp, build a seeds vec (the inputs to the chain)
    //! and a HashMap of the map name to the maps items as a string.
    let input_str = read_file_to_string(fp);
    let mut final_string = String::new();
    let mut almanac_maps = HashMap::new();
    //For easier parsing, replace the empty lines with '|'.
    for line in input_str.lines() {
        if !line.is_empty() {
            final_string.push_str(line);
            final_string.push('\n');
        } else {
            final_string.push('|');
        }
    }
    for partition in final_string.split('|') {
        let mut partition_splitter = partition.split(':');
        let name = partition_splitter.next().unwrap().trim();
        let map = partition_splitter.next().unwrap().trim();
        almanac_maps.insert(name.to_owned(), map.to_owned());
    }

    let seeds = almanac_maps.remove("seeds").unwrap();
    let seeds = seeds
        .split(' ')
        .map(|item| item.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    (seeds, almanac_maps)
}

fn process_maps_into_numbers(
    almanac_map: HashMap<String, String>,
) -> HashMap<String, Vec<(usize, usize, usize)>> {
    //! Given the almanac_map, process the strings into vecs of numbers.
    let mut processed_alamanac_map = HashMap::new();

    for (map_name, map) in almanac_map.into_iter() {
        let mut range_vec = Vec::new();
        for line in map.lines() {
            let mut line_iterator = line
                .split(' ')
                .map(|item| item.trim().parse::<usize>().unwrap());
            let dest_range_start = line_iterator.next().unwrap();
            let src_range_start = line_iterator.next().unwrap();
            let range = line_iterator.next().unwrap();
            range_vec.push((src_range_start, range, dest_range_start));
        }
        processed_alamanac_map.insert(map_name, range_vec);
    }
    processed_alamanac_map
}

fn get_single_map_value(
    map_key: &str,
    input_int: usize,
    almanac_maps: &HashMap<String, Vec<(usize, usize, usize)>>,
) -> usize {
    //! Given the desired map to use, the input integer, and the almanac,
    //! use the rules to return the correct value.
    let range_iterator = almanac_maps.get(map_key).unwrap();
    for (src_start, range, dest_start) in range_iterator {
        //Iterate over each item in the map
        if &input_int >= src_start && input_int < src_start + range {
            //If the input falls in the input range, then the correct return value
            //is the dest_start plus how far from the src_start the input_int is.
            let delta = input_int - src_start;
            return dest_start + delta;
        }
    }
    //If its not in the map, then the correct value is the input_id
    input_int
}

fn apply_chain(seed_input: usize, maps: &HashMap<String, Vec<(usize, usize, usize)>>) -> usize {
    //! Given the seed_input, apply the chain of maps, returning the location_value.
    let soil_value = get_single_map_value("seed-to-soil map", seed_input, maps);
    let fertilizer_value = get_single_map_value("soil-to-fertilizer map", soil_value, maps);
    let water_value = get_single_map_value("fertilizer-to-water map", fertilizer_value, maps);
    let light_value = get_single_map_value("water-to-light map", water_value, maps);
    let temperature_value = get_single_map_value("light-to-temperature map", light_value, maps);
    let humidity_value =
        get_single_map_value("temperature-to-humidity map", temperature_value, maps);
    get_single_map_value("humidity-to-location map", humidity_value, maps)
}
