use crate::{read_file_to_string, SolveAdvent};

pub struct Day9;

impl SolveAdvent for Day9 {
    fn solve_part1(path_to_file: &str) {
        let file_as_str = read_file_to_string(path_to_file);
        let mut adder_total = 0;
        for line in file_as_str.lines() {
            let line_split = line
                .split(' ')
                .into_iter()
                .map(|item| item.trim().parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            adder_total += extrapolate_history_part1(line_split);
        }
        println!("Final Adder total: {}", adder_total);
    }

    fn solve_part2(path_to_file: &str) {
        let file_as_str = read_file_to_string(path_to_file);
        let mut adder_total = 0;
        for line in file_as_str.lines() {
            let line_split = line
                .split(' ')
                .into_iter()
                .map(|item| item.trim().parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            adder_total += extrapolate_history_part2(line_split);
        }
        println!("Final Adder total: {}", adder_total);
    }
}

fn build_history_pyramid(history: Vec<i32>) -> Vec<Vec<i32>> {
    //! Build the history pyramid, where each row is the delta of the row before it.
    //! Stop when you get a row of all zeros.
    let mut history_pyramid = vec![history];
    while !history_pyramid[history_pyramid.len() - 1]
        .iter()
        .all(|item| item == &0)
    {
        let last_history = history_pyramid.get(history_pyramid.len() - 1).unwrap();
        //Build a new vector out of the delta of each item in the pyramid row above.
        let mut new_pyramid = Vec::new();
        for i in 0..last_history.len() - 1 {
            new_pyramid.push(last_history[i + 1] - last_history[i]);
        }
        history_pyramid.push(new_pyramid);
    }
    history_pyramid
}

fn extrapolate_history_part1(history: Vec<i32>) -> i32 {
    //Step1: build the pyramid shown in the advent calendar example, stop
    //building the pyramid when all of the items are 0.
    let mut history_pyramid = build_history_pyramid(history);

    let mut adder = 0;
    //Remove the bottom row, because we know its all zeros anyway.
    history_pyramid.pop().unwrap();
    while !history_pyramid.is_empty() {
        let last_history = history_pyramid.pop().unwrap();
        //Set the new adder to the last value in the bottom row of the pyramid plus the old adder
        adder = last_history.iter().last().unwrap() + adder;
    }
    //When the loop completes, the adder value is now the extrapolated history.
    adder
}

fn extrapolate_history_part2(history: Vec<i32>) -> i32 {
    //Essentially the exact same logic as part1, with small adjustments for getting a backwards history
    //this time.
    let mut history_pyramid = build_history_pyramid(history);

    let mut subtractor = 0;
    history_pyramid.pop().unwrap();
    while !history_pyramid.is_empty() {
        let last_history = history_pyramid.pop().unwrap();
        subtractor = last_history.iter().next().unwrap() - subtractor;
    }
    subtractor
}
