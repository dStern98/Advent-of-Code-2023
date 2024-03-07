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

        println!("Minimum location value for part1 is {}", min_seen);
    }

    fn solve_part2(path_to_file: &str) {
        let (seeds, maps_hashmap) = process_into_maps(path_to_file);
        let processed_maps = process_maps_into_numbers(maps_hashmap);
        let minimum_location = find_location_using_ranges(seeds, &processed_maps);
        println!("Minimum location value for part2 is {}", minimum_location);
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
    // ! Given the seed_input, apply the chain of maps, returning the location_value.
    let soil_value = get_single_map_value("seed-to-soil map", seed_input, maps);
    let fertilizer_value = get_single_map_value("soil-to-fertilizer map", soil_value, maps);
    let water_value = get_single_map_value("fertilizer-to-water map", fertilizer_value, maps);
    let light_value = get_single_map_value("water-to-light map", water_value, maps);
    let temperature_value = get_single_map_value("light-to-temperature map", light_value, maps);
    let humidity_value =
        get_single_map_value("temperature-to-humidity map", temperature_value, maps);
    get_single_map_value("humidity-to-location map", humidity_value, maps)
}

fn generate_seed_ranges(mut seed_ranges: Vec<usize>) -> Vec<(usize, usize)> {
    //! Process the input as an inclusive range of seed numbers.
    let seed_drain = &mut seed_ranges.drain(..);
    let mut seed_ranges = Vec::new();
    loop {
        let next_seeds = seed_drain.take(2).collect::<Vec<_>>();
        if next_seeds.is_empty() {
            break;
        }
        let lower_seed_number = next_seeds[0];
        let step_by = next_seeds[1];
        seed_ranges.push((lower_seed_number, lower_seed_number + step_by - 1));
    }
    seed_ranges
}

fn find_location_using_ranges(
    seed_ranges: Vec<usize>,
    maps: &HashMap<String, Vec<(usize, usize, usize)>>,
) -> usize {
    //! Process all of the ranges from each of the maps one after the other.
    //! If done correctly, the final map maps the range of seeds to the range
    //! of locations, which allows for trivial computation of the minimum possible
    //! location.
    let map_names = [
        "seed-to-soil map",
        "soil-to-fertilizer map",
        "fertilizer-to-water map",
        "water-to-light map",
        "light-to-temperature map",
        "temperature-to-humidity map",
        "humidity-to-location map",
    ];

    let paired_seeds = generate_seed_ranges(seed_ranges);
    let mut min_seen = usize::MAX;

    //Initialize the current_range_map to contain a mapping of the seed intervals to themselves.
    //Each iteration of the outermost for loop changes what the current_range_map maps. The keys are always
    //the seed ranges, but the values become the ranges of the next map once applied.
    //So after the first loop, the current_range_map changes from a map of seeds to seed to a map of
    //seeds to soil.
    let mut current_range_map = paired_seeds
        .into_iter()
        .map(|pair| (pair, pair))
        .collect::<HashMap<(usize, usize), (usize, usize)>>();
    for map_key in map_names.iter() {
        //Drain the current map
        let drained_current_map = current_range_map.drain().collect::<Vec<_>>();
        for (current_src, current_dest) in drained_current_map.iter() {
            let current_src_range = Range::new(current_src.0, current_src.1);
            let current_dest_range = Range::new(current_dest.0, current_dest.1);
            let mut next_collected_map_src = Vec::new();
            for (src_start, range, dest_start) in maps.get(*map_key).unwrap() {
                let next_src_range = Range::new(*src_start, src_start + range - 1);
                let next_dest_range = Range::new(*dest_start, dest_start + range - 1);
                current_range_map.extend(apply_range_mapping(
                    current_src_range.clone(),
                    current_dest_range.clone(),
                    next_src_range.clone(),
                    next_dest_range,
                ));
                //Store the next_src_range's for later.
                next_collected_map_src.push(next_src_range);
            }

            current_range_map.extend(fill_unmapped_ranges(
                current_src_range,
                current_dest_range,
                next_collected_map_src,
            ));
        }
    }
    //The lowest number in the values of the current_range_map are the minimum
    //possible location.
    for (lower_location, _) in current_range_map.values() {
        if lower_location < &min_seen {
            min_seen = *lower_location;
        }
    }

    min_seen
}

///Simple Range type to help with reasoning
/// regarding range operations. Most importantly, both the
/// lower and upper fields are inclusive.
#[derive(Clone, Debug, PartialEq)]
struct Range {
    lower: isize,
    upper: isize,
}

impl Range {
    fn new(lower: usize, upper: usize) -> Range {
        Range {
            lower: lower as isize,
            upper: upper as isize,
        }
    }

    fn overlaps(&self, other: &Range) -> bool {
        //! Whether or not two ranges overlap.
        self.lower.max(other.lower) <= self.upper.min(other.upper)
    }

    fn into_ordered_pair(self) -> (usize, usize) {
        //! Convert the Range type back into a tuple.
        (self.lower as usize, self.upper as usize)
    }

    fn is_valid_range(&self) -> bool {
        //! A valid range must have the lower field be gte the upper field.
        //! This method allows for vastly simplified logic.
        self.lower <= self.upper
    }
}

fn apply_range_mapping(
    current_map_src: Range,
    current_map_dest: Range,
    next_map_src: Range,
    next_map_dest: Range,
) -> HashMap<(usize, usize), (usize, usize)> {
    //! Apply the basic merge operation of the old range map to the new range map.
    //! For example, suppose the current_map was (15,22) -> (36, 43) and the next_map
    //! was (30-37) -> (100-107). The output should be:
    //! (21, 22) -> (106, 107). Note that the src will always be a subset of the original src, but the
    //! dest has likely changed completely. Importantly, the requirement that unmapped ranges return themselves
    //! cannot be handled here, because each call of this function does not know if the next map_src will match.
    let mut outbound_map = HashMap::with_capacity(1);
    let merged_map = Range {
        lower: current_map_dest.lower.max(next_map_src.lower),
        upper: current_map_dest.upper.min(next_map_src.upper),
    };
    if !merged_map.is_valid_range() {
        return outbound_map;
    }
    let new_src_range = Range {
        lower: current_map_src.lower + merged_map.lower - current_map_dest.lower,
        upper: current_map_src.upper + merged_map.upper - current_map_dest.upper,
    };
    let new_dest_range = Range {
        lower: next_map_dest.lower + merged_map.lower - next_map_src.lower,
        upper: next_map_dest.upper + merged_map.upper - next_map_src.upper,
    };
    outbound_map.insert(
        new_src_range.into_ordered_pair(),
        new_dest_range.into_ordered_pair(),
    );
    outbound_map
}

fn fill_unmapped_ranges(
    current_src_range: Range,
    current_dest_range: Range,
    next_src_ranges: Vec<Range>,
) -> HashMap<(usize, usize), (usize, usize)> {
    //! The problem description requires that unmapped ranges be mapped to themselves.
    //! This function accomplishes this by starting with a single hole representing the entire
    //! destination range. Iterating over the next_src_ranges, the ranges known to not be holes
    //! are removed. When the iteration is complete, any remaining holes are real holes, that need
    //! to be mapped to themselves.
    let mut potential_holes = vec![current_dest_range.clone()];
    for next_src_range in next_src_ranges {
        let mut counter = 0;
        while counter < potential_holes.len() {
            let current_hole = potential_holes.get(counter).unwrap();
            if current_hole.overlaps(&next_src_range) {
                let new_holes = [
                    Range {
                        lower: current_hole.lower.min(next_src_range.lower),
                        upper: current_hole.lower.max(next_src_range.lower) - 1,
                    },
                    Range {
                        lower: current_hole.upper.min(next_src_range.upper) + 1,
                        upper: current_hole.upper.max(next_src_range.upper),
                    },
                ];

                let new_holes = new_holes
                    .into_iter()
                    .filter(|range| range.overlaps(current_hole))
                    .collect::<Vec<_>>();
                potential_holes.remove(counter);
                if !new_holes.is_empty() {
                    potential_holes.extend(new_holes);
                    break;
                }
            }
            counter += 1;
        }
    }
    let mut unmapped_range_map = HashMap::new();
    for remaining_range in potential_holes {
        let delta_lower = remaining_range.lower - current_dest_range.lower;
        let delta_upper = remaining_range.upper - current_dest_range.upper;
        unmapped_range_map.insert(
            Range {
                lower: current_src_range.lower + delta_lower,
                upper: current_src_range.upper + delta_upper,
            }
            .into_ordered_pair(),
            remaining_range.clone().into_ordered_pair(),
        );
    }
    unmapped_range_map
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_merge_maps1() {
        let current_map_src = Range::new(5, 12);
        let current_map_dest = Range::new(16, 23);
        let next_map_src = Range::new(18, 21);
        let next_map_dest = Range::new(101, 104);
        let merged_map = apply_range_mapping(
            current_map_src,
            current_map_dest,
            next_map_src,
            next_map_dest,
        );
        assert_eq!(merged_map.len(), 1);
        assert_eq!(merged_map.get(&(7, 10)).unwrap(), &(101, 104));
    }

    #[test]
    fn test_merge_maps2() {
        let current_map_src = Range::new(5, 8);
        let current_map_dest = Range::new(9, 12);
        let next_map_src = Range::new(8, 10);
        let next_map_dest = Range::new(42, 44);
        let merged_map = apply_range_mapping(
            current_map_src,
            current_map_dest,
            next_map_src,
            next_map_dest,
        );
        assert_eq!(merged_map.len(), 1);
        assert_eq!(merged_map.get(&(5, 6)).unwrap(), &(43, 44));
    }
    #[test]
    fn test_merge_maps3() {
        let current_map_src = Range::new(7, 9);
        let current_map_dest = Range::new(8, 10);
        let next_map_src = Range::new(11, 13);
        let next_map_dest = Range::new(12, 14);
        let merged_map = apply_range_mapping(
            current_map_src,
            current_map_dest,
            next_map_src,
            next_map_dest,
        );
        assert_eq!(merged_map.len(), 0);
    }

    #[test]
    fn test_hole_filling1() {
        let holes = fill_unmapped_ranges(
            Range::new(8, 20),
            Range::new(8, 20),
            vec![Range::new(9, 13), Range::new(15, 17), Range::new(18, 20)],
        );
        assert_eq!(holes.len(), 2);
        assert!(holes.contains_key(&(8, 8)));
        assert!(holes.contains_key(&(14, 14)));
    }

    #[test]
    fn test_hole_filling2() {
        let holes = fill_unmapped_ranges(
            Range::new(8, 20),
            Range::new(8, 20),
            vec![Range::new(7, 21)],
        );
        assert_eq!(holes.len(), 0);
    }
    #[test]
    fn test_hole_filling3() {
        let holes = fill_unmapped_ranges(
            Range::new(8, 20),
            Range::new(8, 20),
            vec![Range::new(9, 21)],
        );
        assert_eq!(holes.len(), 1);
        assert!(holes.contains_key(&(8, 8)));
    }
    #[test]
    fn test_hole_filling4() {
        let holes = fill_unmapped_ranges(
            Range::new(8, 20),
            Range::new(8, 20),
            vec![Range::new(7, 15)],
        );
        assert_eq!(holes.len(), 1);
        assert!(holes.contains_key(&(16, 20)));
    }
    #[test]
    fn test_hole_filling5() {
        let holes = fill_unmapped_ranges(
            Range::new(15, 27),
            Range::new(15, 27),
            vec![Range::new(8, 15)],
        );
        assert_eq!(holes.len(), 1);
        assert!(holes.contains_key(&(16, 27)));
    }

    #[test]
    fn test_overlap_assessment() {
        let range1 = Range::new(8, 20);
        let range2 = Range::new(13, 15);
        let range3 = Range::new(17, 25);
        let range4 = Range::new(5, 10);
        assert!(range1.overlaps(&range2));
        assert!(range1.overlaps(&range3));
        assert!(range1.overlaps(&range4));
    }
}
