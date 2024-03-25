use crate::{read_input_file, SolveAdvent};

pub struct Day6;

impl SolveAdvent for Day6 {
    fn solve_part1(path_to_file: &str) {
        let files_as_str = read_input_file(path_to_file);
        let processed_race_numbers = process_input_file(files_as_str);
        let mut multiplicative_ways_to_win = 1;
        for (required_time, required_distance) in processed_race_numbers {
            let ways_to_win = ways_to_win_race(required_distance, required_time);
            multiplicative_ways_to_win *= ways_to_win;
        }
        println!("Multiplicative Ways to Win: {}", multiplicative_ways_to_win);
    }

    fn solve_part2(path_to_file: &str) {
        let files_as_str = read_input_file(path_to_file);
        let (required_time, required_distance) = process_input_file_part2(files_as_str);
        let ways_to_win = ways_to_win_race(required_distance, required_time);
        println!("Ways to win single race: {}", ways_to_win);
    }
}

fn ways_to_win_race(required_distance: usize, required_time: usize) -> usize {
    let mut ways_to_win = 0;
    for time_holding_button in 1..required_time {
        let time_to_move = required_time - time_holding_button;
        let distance_traveled = time_to_move * time_holding_button;
        if distance_traveled > required_distance {
            ways_to_win += 1;
        }
    }
    ways_to_win
}

fn process_input_file(file_as_str: String) -> Vec<(usize, usize)> {
    //! Process the input String into zipped pairs of time and distance.
    let mut lines_iterator = file_as_str.lines();
    let times = lines_iterator.next().unwrap().trim();
    let distances = lines_iterator.next().unwrap().trim();
    let times = times
        .replace("Time:", "")
        .trim()
        .split(' ')
        .filter_map(|item| {
            if !item.is_empty() {
                return Some(item.trim().parse::<usize>().unwrap());
            }
            None
        })
        .collect::<Vec<_>>();
    let distances = distances
        .replace("Distance:", "")
        .trim()
        .split(' ')
        .filter_map(|item| {
            if !item.is_empty() {
                return Some(item.trim().parse::<usize>().unwrap());
            }
            None
        })
        .collect::<Vec<_>>();

    times.into_iter().zip(distances).collect::<Vec<_>>()
}

fn process_input_file_part2(file_as_str: String) -> (usize, usize) {
    //! Process the input String into the single time and distance.
    let mut lines_iterator = file_as_str.lines();
    let times = lines_iterator.next().unwrap().trim();
    let distances = lines_iterator.next().unwrap().trim();
    let time = times
        .replace("Time:", "")
        .trim()
        .replace(' ', "")
        .parse::<usize>()
        .unwrap();
    let distance = distances
        .replace("Distance:", "")
        .trim()
        .replace(' ', "")
        .parse::<usize>()
        .unwrap();
    (time, distance)
}
